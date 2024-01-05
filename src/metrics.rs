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
//!         start_http_server(9090).await;
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

use log::error;
use warp::{Filter, Rejection, Reply};

pub use lazy_static::lazy_static;
pub extern crate lazy_static;

pub use prometheus::*;

use crate::error::DshError;

/// Encode metrics to a string (UTF8)
pub fn metrics_to_string() -> std::result::Result<String, DshError> {
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    encoder.encode(&prometheus::gather(), &mut buffer)?;
    let res = String::from_utf8(buffer)?;
    Ok(res)
}

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
///       start_http_server(8080).await;
///   });
/// }
/// ```
pub async fn start_http_server(port: u16) {
    let metrics_route = warp::path!("metrics").and_then(http_metric_response);
    warp::serve(metrics_route).run(([0, 0, 0, 0], port)).await;
}

/// Function for warp to handle http request.
///
/// Calls the metrics_to_string function to get the metrics as string and returns them as a reply
async fn http_metric_response() -> std::result::Result<impl Reply, Rejection> {
    let res = match metrics_to_string() {
        Ok(v) => v,
        Err(e) => {
            error!("Prometheus metrics could not be gathered: {}", e);
            String::default()
        }
    };
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::Response;
    use warp::test::request;

    lazy_static! {
        pub static ref HIGH_FIVE_COUNTER: IntCounter =
            register_int_counter!("highfives", "Number of high fives recieved").unwrap();
    }

    #[tokio::test]
    async fn test_http_metric_response() {
        // Increment the counter
        HIGH_FIVE_COUNTER.inc();

        // Call the function
        let res = http_metric_response().await;

        // Check if the function returns a result
        assert!(res.is_ok());

        // Check if the result is not an empty string
        let status_code = res.unwrap().into_response().status();
        assert_eq!(
            status_code,
            Response::builder().status(200).body(()).unwrap().status()
        );
    }

    #[tokio::test]
    async fn test_start_http_server() {
        // Spawn the server in a separate task
        let server = tokio::spawn(async {
            start_http_server(8080).await;
        });

        // increment the counter
        HIGH_FIVE_COUNTER.inc();

        // Give the server a moment to start
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Send a request to the server
        let res = request()
            .method("GET")
            .path("/metrics")
            .reply(&warp::path!("metrics").and_then(http_metric_response))
            .await;

        // Check if the server returns a 200 status
        assert_eq!(res.status(), 200);

        // Check if the response is not an empty string
        assert!(!res.body().is_empty());

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
