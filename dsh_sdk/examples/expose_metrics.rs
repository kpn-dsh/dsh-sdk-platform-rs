use dsh_sdk::utils::metrics::start_http_server;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter};
use std::sync::OnceLock;

// Register counter with lazy_static
// (not reccomended to use lazy_static)
lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives recieved").unwrap();
}

// Register counter with Rust std library
// Recomended way to register metrics
pub fn low_five_counter() -> &'static IntCounter {
    static CONSUMED_MESSAGES: OnceLock<IntCounter> = OnceLock::new();
    CONSUMED_MESSAGES.get_or_init(|| {
        register_int_counter!("consumed_messages", "Number of messages consumed").unwrap()
    })
}

/// Gather and encode metrics to a string (UTF8)
pub fn encode_metrics() -> String {
    let encoder = prometheus::TextEncoder::new();
    encoder
        .encode_to_string(&prometheus::gather())
        .unwrap_or_default()
}

#[tokio::main]
async fn main() {
    println!("Starting metrics server on http://localhost:8080/metrics");
    start_http_server(8080, encode_metrics);

    // increment the counters every second for 20 times
    for i in 0..20 {
        println!("High five number: {}", i + 1);
        HIGH_FIVE_COUNTER.inc();
        println!("Low five number: {}", i + 1);
        low_five_counter().inc();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
