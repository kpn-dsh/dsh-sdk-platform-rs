use dsh_sdk::metrics::*;

lazy_static! {
    pub static ref CONSUMED_MESSAGES: IntCounter =
        register_int_counter!("consumed_messages", "Number of messages consumed").unwrap();
}
