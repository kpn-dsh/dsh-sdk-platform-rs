use prometheus::{register_int_counter, IntCounter};
use std::sync::OnceLock;

/// Counter for consumed messages
pub fn consumed_messages() -> &'static IntCounter {
    static CONSUMED_MESSAGES: OnceLock<IntCounter> = OnceLock::new();
    CONSUMED_MESSAGES.get_or_init(|| {
        register_int_counter!("consumed_messages", "Number of messages consumed").unwrap()
    })
}

/// Gather and encode the metrics to string
pub fn gather_and_encode() -> String {
    let encoder = prometheus::TextEncoder::new();
    encoder
        .encode_to_string(&prometheus::gather())
        .unwrap_or_default()
}
