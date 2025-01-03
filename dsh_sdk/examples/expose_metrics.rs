use dsh_sdk::utils::metrics::*;

lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives recieved").unwrap();
}

#[tokio::main]
async fn main() {
    println!("Starting metrics server on http://localhost:8080/metrics");
    start_http_server(8080);

    // increment the high five counter every second for 20 times
    for i in 0..20 {
        println!("High five number: {}", i + 1);
        HIGH_FIVE_COUNTER.inc();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
