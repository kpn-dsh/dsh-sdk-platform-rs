//! Provides a lightweight HTTP server to expose (prometheus) metrics.
//!
//! ## Expose metrics to DSH / HTTP Server
//!
//! This module provides a http server to expose the metrics to DSH. A port number and a function that encode the metrics to [String] needs to be defined.
//!
//! Most metrics libraries provide a way to encode the metrics to a string. For example,
//!  - [prometheus-client](https://crates.io/crates/prometheus-client) library provides a [render](https://docs.rs/prometheus-client/latest/prometheus_client/encoding/text/index.html) function to encode the metrics to a string.
//!  - [prometheus](https://crates.io/crates/prometheus) library provides a [TextEncoder](https://docs.rs/prometheus/latest/prometheus/struct.TextEncoder.html) to encode the metrics to a string.
//! See [expose_metrics.rs](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/expose_metrics.rs) for a full example implementation.
//!
//! ### Example:
//! ```
//! use dsh_sdk::utils::metrics::start_http_server;
//!
//! fn encode_metrics() -> String {
//!     // Provide here your logic to gather and encode the metrics to a string
//!     // Check your chosen metrics library for the correct implementation
//!     "my_metrics 1".to_string() // Dummy example
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!    start_http_server(9090, encode_metrics);
//!}
//! ```
//! After starting the http server, the metrics can be found at http://localhost:9090/metrics.
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
use log::{error, warn};
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

static NOTFOUND: &[u8] = b"404: Not Found";

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetricsError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::http::Error),
}

/// A lihghtweight HTTP server to expose prometheus metrics.
///
/// The exposed endpoint is /metrics and port number needs to be defined together with your gather and encode function to string.
/// The server will run on a separate thread and this function will return a JoinHandle of the thread.
/// It is optional to handle the thread status. If left unhandled, the server will run until the main thread is stopped.
///
/// # Example
/// This starts a http server on port 9090 on a separate thread. The server will run until the main thread is stopped.
///  ```
///  use dsh_sdk::utils::metrics::start_http_server;
///  
///  fn encode_metrics() -> String {
///      // Provide here your logic to gather and encode the metrics to a string
///      // Check your chosen metrics library for the correct implementation
///      "my_metrics 1".to_string() // Dummy example
///  }
///  
///  #[tokio::main]
///  async fn main() {
///     start_http_server(9090, encode_metrics);
/// }
///  ```
///
/// ## Optional: Check http server thread status
/// Await the JoinHandle in a a tokio select besides your application logic to check if the server is still running.
/// ```rust
/// # use dsh_sdk::utils::metrics::start_http_server;
/// # use tokio::time::sleep;
/// # use std::time::Duration;
/// # fn encode_metrics() -> String {
/// #     "my_metrics 1".to_string() // Dummy example
/// # }
/// # #[tokio::main]
/// # async fn main() {
/// let server = start_http_server(9090, encode_metrics);
/// tokio::select! {
///      // Replace sleep with your application logic
///      _ = sleep(Duration::from_secs(1)) => {println!("Application is stoped!")},
///      // Check if the server is still running
///      tokio_result = server => {
///          match tokio_result   {
///              Ok(server_result) => if let Err(e) = server_result {
///                  eprintln!("Metrics server operation failed: {}", e);
///              },
///              Err(e) => println!("Server thread stopped unexpectedly: {}", e),
///          }
///      }
/// }
/// # }
/// ```
pub fn start_http_server(
    port: u16,
    metrics_encode_fn: fn() -> String,
) -> JoinHandle<Result<(), MetricsError>> {
    let server = MetricsServer {
        port,
        metrics_encode_fn,
    };
    tokio::spawn(async move {
        let result = server.run_server().await;
        warn!("HTTP server stopped: {:?}", result);
        result
    })
}

struct MetricsServer {
    port: u16,
    metrics_encode_fn: fn() -> String,
}

impl MetricsServer {
    async fn run_server(&self) -> Result<(), MetricsError> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            self.handle_connection(stream).await;
        }
    }

    async fn handle_connection(&self, stream: tokio::net::TcpStream) {
        let io = TokioIo::new(stream);
        let service = service_fn(|req| self.routes(req));
        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            error!("Failed to serve metrics connection: {:?}", err);
        }
    }

    async fn routes(&self, req: Request<Incoming>) -> Result<Response<BoxBody>, MetricsError> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/metrics") => self.get_metrics(),
            (_, _) => not_found(),
        }
    }

    fn get_metrics(&self) -> Result<Response<BoxBody>, MetricsError> {
        let body = (self.metrics_encode_fn)();
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(full(body))?)
    }
}

fn not_found() -> Result<Response<BoxBody>, MetricsError> {
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
    use hyper::Uri;
    use lazy_static::lazy_static;
    use prometheus::{register_int_counter, IntCounter};
    use serial_test::serial;
    use tokio::net::TcpStream;

    const PORT: u16 = 9090;

    /// Gather and encode metrics to a string (UTF8)
    pub fn metrics_to_string() -> String {
        let encoder = prometheus::TextEncoder::new();
        encoder
            .encode_to_string(&prometheus::gather())
            .unwrap_or_default()
    }

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

        let server = MetricsServer {
            port: PORT,
            metrics_encode_fn: metrics_to_string,
        };
        // Call the function
        let res = server.get_metrics();

        // Check if the function returns a result
        assert!(res.is_ok());

        // Check if the result is not an empty string
        let response = res.unwrap();
        let status_code = response.status();

        assert_eq!(status_code, StatusCode::OK);
        assert!(response.body().size_hint().exact().unwrap() > 0);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/plain"
        );
    }

    #[tokio::test]
    #[serial(port_usage)]
    async fn test_start_http_server() {
        // Start HTTP server
        let server = start_http_server(PORT, metrics_to_string);

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
            "text/plain"
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
        let server = start_http_server(PORT, metrics_to_string);

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
}
