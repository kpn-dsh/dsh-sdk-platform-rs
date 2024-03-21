use dsh_sdk::metrics::*;
use prometheus::{register_int_gauge_vec, IntGaugeVec};

lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives recieved").unwrap();
    pub static ref SPEEDOMETRE: IntGaugeVec =
        register_int_gauge_vec!("speedometre", "Speedometre", &["type"]).unwrap();
}

fn main() {
    // increment the high five counter
    HIGH_FIVE_COUNTER.inc();
    // set the speed to 100
    SPEEDOMETRE.with_label_values(&["speed"]).set(100);
    // simple print statement to show the metrics in prometheus format
    println!("{}", metrics_to_string().unwrap())
}
