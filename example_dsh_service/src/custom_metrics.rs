use dsh_sdk::utils::metrics::*;

lazy_static! {
    pub static ref CONSUMED_MESSAGES: IntCounter =
        register_int_counter!("consumed_messages", "Number of messages consumed").unwrap();
}
