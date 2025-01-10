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
