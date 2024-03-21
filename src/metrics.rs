//! This module wraps the prometheus metrics library and provides a http server to expose the metrics.
//!
//! It is technically a re-exports the prometheus metrics library with some additional functions.
//!
//! # Create custom metrics
//!
//! To define custom metrics, the prometheus macros can be used. They are re-exported in this module.
//!
//! As they are a pub static reference, you can use them anywhere in your code.
//!
//! See [prometheus](https://docs.rs/prometheus/0.13.3/prometheus/index.html#macros) for more information.
//!
//! ### Example
//! ```
//! use dsh_sdk::metrics::*;
//!
//! lazy_static! {
//!     pub static ref HIGH_FIVE_COUNTER: IntCounter =
//!         register_int_counter!("highfives", "Number of high fives recieved").unwrap();
//! }
//!
//! HIGH_FIVE_COUNTER.inc();
//! ```
//!
//! # Expose metrics to DSH / HTTP Server
//!
//! This module provides a http server to expose the metrics to DSH. A port number needs to be defined.
//!
//! ### Example:
//! ```
//! use dsh_sdk::metrics::start_http_server;
//!
//! #[tokio::main]
//! async fn main() {
//!     tokio::spawn(async move {
//!         start_http_server(9090).await.unwrap();
//!    });
//! }
//! ```
//! After starting the http server, the metrics can be found at http://localhost:8080/metrics.
//! To expose the metrics to DSH, the port number needs to be defined in the DSH service configuration.
//!
//! ```json
//! "metrics": {
//!     "port": 9090,
//!     "path": "/metrics"
//! },
//! ```
//!
//! And in your dockerfile expose the port:
//! ```dockerfile
//! EXPOSE 9090
//! ```

use std::net::SocketAddr;

use crate::error::DshError;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{header, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
pub use lazy_static::lazy_static;
use log::error;
pub use prometheus::register_int_counter;
use prometheus::Encoder;
pub use prometheus::IntCounter;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

static NOTFOUND: &[u8] = b"Not Found";

/// Start a http server to expose prometheus metrics.
///
/// The exposed endpoint is /metrics and port number needs to be defined
///
/// # Note!
///
/// Don't forget to expose the port in your dockerfile and add the port number to the DSH service configuration.
///
/// # Example
/// ```
/// use dsh_sdk::metrics::start_http_server;
/// #[tokio::main]
/// async fn main() {
///    tokio::spawn(async move {
///       start_http_server(8080).await.unwrap();
///   });
/// }
/// ```
///
pub async fn start_http_server(port: u16) -> Result<()> {
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let service = service_fn(move |req| routes(req));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                error!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn routes(req: Request<Incoming>) -> Result<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => get_metrics(),
        (_, _) => not_found(),
    }
}

fn get_metrics() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, prometheus::TEXT_FORMAT)
        .body(full(metrics_to_string().unwrap_or_default()))
        .unwrap())
}

/// Encode metrics to a string (UTF8)
pub fn metrics_to_string() -> std::result::Result<String, DshError> {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    encoder.encode(&prometheus::gather(), &mut buffer)?;
    let res = String::from_utf8(buffer)?;
    Ok(res)
}

fn not_found() -> Result<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(NOTFOUND))
        .unwrap())
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[cfg(test)]
mod tests {
    use http_body_util::Empty;
    use hyper::body::Body;
    use hyper::client::conn;
    use hyper::client::conn::http1::{Connection, SendRequest};
    use hyper::http::HeaderValue;
    use hyper::Uri;
    use tokio::net::TcpStream;

    use super::*;

    lazy_static! {
        pub static ref HIGH_FIVE_COUNTER: IntCounter =
            register_int_counter!("highfives", "Number of high fives recieved").unwrap();
    }

    async fn create_client(
        url: &Uri,
    ) -> (
        SendRequest<Empty<Bytes>>,
        Connection<TokioIo<TcpStream>, Empty<Bytes>>,
    ) {
        let host = url.host().expect("uri has no host");
        let port = url.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await.unwrap();
        let io = TokioIo::new(stream);

        conn::http1::handshake(io).await.unwrap()
    }

    fn to_get_req(url: &Uri) -> Request<Empty<Bytes>> {
        Request::builder()
            .uri(url)
            .method(Method::GET)
            .header(header::HOST, url.authority().unwrap().clone().as_str())
            .body(Empty::<Bytes>::new())
            .unwrap()
    }

    #[tokio::test]
    async fn test_http_metric_response() {
        // Increment the counter
        HIGH_FIVE_COUNTER.inc();

        // Call the function
        let res = get_metrics();

        // Check if the function returns a result
        assert!(res.is_ok());

        // Check if the result is not an empty string
        let response = res.unwrap();
        let status_code = response.status();

        assert_eq!(status_code, StatusCode::OK);
        assert!(response.body().size_hint().exact().unwrap() > 0);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            HeaderValue::from_static(prometheus::TEXT_FORMAT)
        );
    }

    #[tokio::test]
    async fn test_start_http_server() {
        // Spawn the server in a separate task
        let server = tokio::spawn(async {
            start_http_server(8080).await.unwrap();
        });

        // increment the counter
        HIGH_FIVE_COUNTER.inc();

        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let url: Uri = "http://0.0.0.0:8080/metrics".parse().unwrap();
        let (mut request_sender, connection) = create_client(&url).await;
        tokio::task::spawn(async move {
            if let Err(err) = connection.await {
                error!("Connection failed: {:?}", err);
            }
        });

        // Send a request to the server
        let request = to_get_req(&url);

        let response = request_sender.send_request(request).await.unwrap();

        // Check if the server returns a 200 status
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            HeaderValue::from_static(prometheus::TEXT_FORMAT)
        );

        // Check if the response body is not empty
        let buf = response.collect().await.unwrap().to_bytes();
        let res = String::from_utf8(buf.to_vec()).unwrap();

        println!("{}", res);
        assert!(!res.is_empty());

        // Terminate the server
        server.abort();
    }

    #[tokio::test]
    async fn test_unknown_path() {
        // Spawn the server in a separate task
        let server = tokio::spawn(async {
            start_http_server(9900).await.unwrap();
        });

        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let url: Uri = "http://0.0.0.0:9900".parse().unwrap();
        let (mut request_sender, connection) = create_client(&url).await;
        tokio::task::spawn(async move {
            if let Err(err) = connection.await {
                error!("Connection failed: {:?}", err);
            }
        });

        // Send a request to the server
        let request = to_get_req(&url);

        let response = request_sender.send_request(request).await.unwrap();

        // Check if the server returns a 200 status
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Check if the response body is not empty
        let buf = response.collect().await.unwrap().to_bytes();
        let res = String::from_utf8(buf.to_vec()).unwrap();

        assert_eq!(res,"Not Found");

        // Terminate the server
        server.abort();
    }

    #[test]
    fn test_metrics_to_string() {
        HIGH_FIVE_COUNTER.inc();
        let res = metrics_to_string().unwrap();
        assert!(res.contains("highfives"));
    }
}
