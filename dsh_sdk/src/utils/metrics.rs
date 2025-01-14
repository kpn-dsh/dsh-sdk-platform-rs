//! Provides a lightweight HTTP server to expose (Prometheus) metrics.
//!
//! This module runs a simple HTTP server that listens on a specified port
//! and serves an endpoint (`/metrics`) which returns a plain-text string
//! representation of your metrics. It can be used to expose metrics to DSH
//! or any Prometheus-compatible monitoring service.
//!
//! # Overview
//!
//! - **Port**: Chosen at runtime; ensure it’s exposed in your container if using Docker.
//! - **Metrics Encoder**: You supply a function that returns a `String` representation
//!   of your metrics (e.g., from a Prometheus client library).
//! - **Thread Model**: The server runs on a separate Tokio task. You can optionally
//!   keep the resulting `JoinHandle` if you want to monitor or manage its lifecycle.
//!
//! # Common Usage
//! 1. **Define a function** that gathers and encodes your metrics to a `String`.  
//! 2. **Call** [`start_http_server`] with the port and your metrics function.  
//! 3. **Access** your metrics at `http://<HOST>:<PORT>/metrics`.  
//! 4. **Configure** your DSH or Docker environment accordingly (if needed).
//!
//! ## Example
//! ```no_run
//! use dsh_sdk::utils::metrics::start_http_server;
//!
//! fn encode_metrics() -> String {
//!     // Provide custom logic to gather and encode metrics into a string.
//!     // Example below is a placeholder.
//!     "my_counter 1".to_string()
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Launch a metrics server on port 9090
//!     start_http_server(9090, encode_metrics);
//!     // The server runs until the main thread stops or is aborted.
//!     // ...
//! }
//! ```
//!
//! Once running, you can query your metrics at `http://localhost:9090/metrics`.  
//!
//! # Configuration with DSH
//!
//! In your DSH service configuration (assuming JSON), specify the port and path for the metrics:
//! ```json
//! "metrics": {
//!     "port": 9090,
//!     "path": "/metrics"
//! },
//! ```
//! Then, in your Dockerfile, be sure to expose that port:
//! ```dockerfile
//! EXPOSE 9090
//! ```
//!
//! # Monitoring the Server Task
//!
//! `start_http_server` spawns a Tokio task which returns a [`JoinHandle`]. You can:
//! - **Ignore** it: The server continues until the main application exits.  
//! - **Await** it to see if the server encounters an error or closes unexpectedly.
//!
//! ```no_run
//! # use dsh_sdk::utils::metrics::start_http_server;
//! # use tokio::time::sleep;
//! # use std::time::Duration;
//! fn encode_metrics() -> String {
//!     "my_metrics 1".to_string() // Dummy example
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let server_handle = start_http_server(9090, encode_metrics);
//!     tokio::select! {
//!         // Some app logic or graceful shutdown condition
//!         _ = sleep(Duration::from_secs(300)) => {
//!             println!("Main application stopping...");
//!         }
//!
//!         // If the metrics server stops unexpectedly, handle the error
//!         result = server_handle => {
//!             match result {
//!                 Ok(Ok(())) => println!("Metrics server finished gracefully."),
//!                 Ok(Err(e)) => eprintln!("Metrics server error: {}", e),
//!                 Err(join_err) => eprintln!("Metrics server thread panicked: {}", join_err),
//!             }
//!         }
//!     }
//!     println!("All done!");
//! }
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

/// Errors that can occur while running the metrics server.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MetricsError {
    /// An I/O error occurred (e.g., binding the port failed).
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    /// An HTTP error occurred while building or sending a response.
    #[error("Hyper error: {0}")]
    HyperError(#[from] hyper::http::Error),
}

/// Starts a lightweight HTTP server to expose Prometheus-like metrics on `"/metrics"`.
///
/// # Parameters
/// - `port`: The port on which the server listens (e.g., `9090`).
/// - `metrics_encode_fn`: A function returning a `String` containing all relevant metrics.
///
/// # Returns
/// A [`JoinHandle`] wrapping a [`Result<(), MetricsError>`]. The server:
/// - Runs until the main process exits or the handle is aborted.  
/// - May exit early if an underlying error (`MetricsError`) occurs.
///
/// # Example
/// ```no_run
/// use dsh_sdk::utils::metrics::start_http_server;
///
/// fn encode_metrics() -> String {
///     // Provide logic that gathers and encodes your metrics as a string.
///     "my_counter 123".to_string()
/// }
///
/// #[tokio::main]
/// async fn main() {
///     start_http_server(9090, encode_metrics);
///     // The server runs in the background until main ends.
/// }
/// ```
///
/// See the module-level docs for more details on usage patterns.
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
        warn!("HTTP metrics server stopped: {:?}", result);
        result
    })
}

/// Internal struct containing server configuration and logic.
struct MetricsServer {
    port: u16,
    metrics_encode_fn: fn() -> String,
}

impl MetricsServer {
    /// Runs the server in a loop, accepting connections and handling them.
    async fn run_server(&self) -> Result<(), MetricsError> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            self.handle_connection(stream).await;
        }
    }

    /// Handles an individual TCP connection by serving HTTP/1.1 requests.
    async fn handle_connection(&self, stream: tokio::net::TcpStream) {
        let io = TokioIo::new(stream);
        let service = service_fn(|req| self.routes(req));
        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            error!("Failed to serve metrics connection: {:?}", err);
        }
    }

    /// Routes requests to the correct handler based on method & path.
    async fn routes(&self, req: Request<Incoming>) -> Result<Response<BoxBody>, MetricsError> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/metrics") => self.get_metrics(),
            _ => not_found(),
        }
    }

    /// Generates a response containing the metrics string.
    fn get_metrics(&self) -> Result<Response<BoxBody>, MetricsError> {
        let body = (self.metrics_encode_fn)();
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(full(body))?)
    }
}

/// Returns a 404 Not Found response.
fn not_found() -> Result<Response<BoxBody>, MetricsError> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(NOTFOUND))?)
}

/// Converts a string (or byte slice) into a boxed HTTP body.
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

    /// Example function to gather metrics from the `prometheus` crate.
    pub fn metrics_to_string() -> String {
        let encoder = prometheus::TextEncoder::new();
        encoder
            .encode_to_string(&prometheus::gather())
            .unwrap_or_default()
    }

    lazy_static! {
        pub static ref HIGH_FIVE_COUNTER: IntCounter =
            register_int_counter!("highfives", "Number of high fives received").unwrap();
    }

    async fn create_client(
        url: &Uri,
    ) -> (
        SendRequest<Empty<Bytes>>,
        Connection<TokioIo<TcpStream>, Empty<Bytes>>,
    ) {
        let host = url.host().expect("URI has no host");
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
            .header(header::HOST, url.authority().unwrap().as_str())
            .body(Empty::new())
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
        let response = server.get_metrics().expect("failed to get metrics");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/plain"
        );
        // Ensure the body is non-empty
        assert!(response.body().size_hint().exact().unwrap() > 0);
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
        assert!(!res.is_empty());

        // Stop the server
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

        // Send a request to the server with no path (i.e., "/")
        let request = to_get_req(&url);
        let response = request_sender.send_request(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Check the 404 body
        let buf = response.collect().await.unwrap().to_bytes();
        let res = String::from_utf8(buf.to_vec()).unwrap();
        assert_eq!(res, String::from_utf8_lossy(NOTFOUND));

        // Stop the server
        server.abort();
    }
}
