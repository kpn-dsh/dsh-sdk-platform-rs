use dsh_sdk::metrics::*;

lazy_static! {
    pub static ref CONSUMED_MESSAGES: IntCounter =
        register_int_counter!("highfives", "Number of high fives recieved").unwrap();
}