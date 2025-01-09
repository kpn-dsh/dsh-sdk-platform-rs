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
//! use dsh_sdk::utils::metrics::*;
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
//! use dsh_sdk::utils::metrics::start_http_server;
//! #[tokio::main]
//! async fn main() {
//!    start_http_server(9090);
//!}
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

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{header, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
pub use lazy_static::lazy_static;
use log::{error, warn};
pub use prometheus::*;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

type MetricsResult<T> = std::result::Result<T, MetricsError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

static NOTFOUND: &[u8] = b"404: Not Found";

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetricsError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::http::Error),
    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),
}

/// Start a http server to expose prometheus metrics.
///
/// The exposed endpoint is /metrics and port number needs to be defined. The server will run on a separate thread
/// and this function will return a JoinHandle of the thread. It is optional to handle the thread status. If left unhandled,
/// the server will run until the main thread is stopped.
///
/// # Note!
/// Don't forget to expose the port in your dockerfile and add the port number to the DSH service configuration.
///```Dockerfile
/// EXPOSE 9090
/// ```
///
/// # Example
/// This starts a http server on port 9090 on a separate thread. The server will run until the main thread is stopped.
/// ```rust
/// use dsh_sdk::metrics::start_http_server;
///
/// #[tokio::main]
/// async fn main() {
///     start_http_server(9090);
/// }
/// ```
///
/// # Optional: Check http server thread status
/// Await the JoinHandle in a a tokio select besides your application logic to check if the server is still running.
/// ```rust
/// use dsh_sdk::metrics::start_http_server;
/// # use tokio::time::sleep;
/// # use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///    let server = start_http_server(9090);
///     tokio::select! {
///         // Replace sleep with your application logic
///         _ = sleep(Duration::from_secs(1)) => {println!("Application is stoped!")},
///         // Check if the server is still running
///         tokio_result = server => {
///             match tokio_result   {
///                 Ok(server_result) => if let Err(e) = server_result {
///                     eprintln!("Metrics server operation failed: {}", e);
///                 },
///                 Err(e) => println!("Server thread stopped unexpectedly: {}", e),
///             }
///         }
///    }
/// }
/// ```
pub fn start_http_server(port: u16) -> JoinHandle<MetricsResult<()>> {
    tokio::spawn(async move {
        let result = run_server(port).await;
        warn!("HTTP server stopped: {:?}", result);
        result
    })
}

/// Encode metrics to a string (UTF8)
pub fn metrics_to_string() -> MetricsResult<String> {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    encoder.encode(&prometheus::gather(), &mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).into_owned())
}

async fn run_server(port: u16) -> MetricsResult<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let io = TokioIo::new(stream);
    let service = service_fn(routes);

    if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
        error!("Failed to serve connection: {:?}", err);
    }
}

async fn routes(req: Request<Incoming>) -> MetricsResult<Response<BoxBody>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => get_metrics(),
        (_, _) => not_found(),
    }
}

fn get_metrics() -> MetricsResult<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, prometheus::TEXT_FORMAT)
        .body(full(metrics_to_string().unwrap_or_default()))?)
}

fn not_found() -> MetricsResult<Response<BoxBody>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(NOTFOUND))?)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::Empty;
    use hyper::body::Body;
    use hyper::client::conn;
    use hyper::client::conn::http1::{Connection, SendRequest};
    use hyper::http::HeaderValue;
    use hyper::Uri;
    use serial_test::serial;
    use tokio::net::TcpStream;

    const PORT: u16 = 9090;

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
        let port = url.port_u16().unwrap_or(PORT);
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
    #[serial(port_usage)]
    async fn test_start_http_server() {
        // Start HTTP server
        let server = start_http_server(PORT);

        // increment the counter
        HIGH_FIVE_COUNTER.inc();

        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let url: Uri = format!("http://localhost:{PORT}/metrics").parse().unwrap();
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
    #[serial(port_usage)]
    async fn test_unknown_path() {
        // Start HTTP server
        let server = start_http_server(PORT);

        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let url: Uri = format!("http://localhost:{PORT}").parse().unwrap();
        let (mut request_sender, connection) = create_client(&url).await;
        tokio::task::spawn(async move {
            if let Err(err) = connection.await {
                error!("Connection failed: {:?}", err);
            }
        });

        // Send a request to the server
        let request = to_get_req(&url);

        let response = request_sender.send_request(request).await.unwrap();

        // Check if the server returns a 404 status
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Check if the response body is not empty
        let buf = response.collect().await.unwrap().to_bytes();
        let res = String::from_utf8(buf.to_vec()).unwrap();

        assert_eq!(res, String::from_utf8_lossy(NOTFOUND));

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
