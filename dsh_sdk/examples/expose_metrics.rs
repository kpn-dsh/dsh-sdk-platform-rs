//! This example shows how to expose metrics using the `dsh_sdk` library.
//! It demonstrates how to register and increment metrics using the Prometheus library
//! and how to start an HTTP server to expose the metrics.
//!
//! Example is using the following crates:
//! - [`dsh_sdk`] with features = ["metrics"] for exposing metrics
//! - [`prometheus`] for metrics
//!
//! To run this example on your local environment:
//! ```bash
//! cargo r --features metrics --example expose_metrics
//! ```

use dsh_sdk::utils::metrics::start_http_server;
use prometheus::{register_int_counter, IntCounter};
use std::sync::OnceLock;

// Create and register counter
pub fn high_five_counter() -> &'static IntCounter {
    static CONSUMED_MESSAGES: OnceLock<IntCounter> = OnceLock::new();
    CONSUMED_MESSAGES.get_or_init(|| {
        register_int_counter!("highfives", "Number of highfives given").unwrap()
    })
}

/// Gather and encode metrics to a string (UTF8)
pub fn encode_metrics() -> String {
    let encoder = prometheus::TextEncoder::new();
    encoder
        .encode_to_string(&prometheus::gather())
        .unwrap_or_default()
}

fn main() {
    println!("Starting metrics server on http://localhost:8080/metrics");
    start_http_server(8080, encode_metrics);

    // increment the counters every second for 20 times
    for i in 0..20 {
        println!("Low five number: {}", i + 1);
        high_five_counter().inc();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
