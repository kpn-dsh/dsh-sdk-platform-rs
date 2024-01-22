use dsh_sdk::graceful_shutdown::Shutdown;

// your process task
async fn process_task(shutdown: Shutdown) {
    loop {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                // Do something here, e.g. consume messages from Kafka
                println!("Still processing the task, press Ctrl+C to exit")
            },
            _ = shutdown.recv() => {
                // shutdown request received, include your shutdown procedure here e.g. close db connection
                println!("Gracefully exiting process_task");
                break;
            },
        }
    }
}

#[tokio::main]
async fn main() {
    // Create shutdown handle
    let shutdown = dsh_sdk::graceful_shutdown::Shutdown::new();
    // Create your process task with a cloned shutdown handle
    let process_task = process_task(shutdown.clone());
    // Spawn your process task in a tokio runtime
    let process_task_handle = tokio::spawn(async move {
        process_task.await;
    });

    // Listen for shutdown request or if process task stopped
    // If your process stops, start shutdown procedure to stop other tasks (if any)
    tokio::select! {
        _ = shutdown.signal_listener() => println!("Exit signal received!"),
        _ = process_task_handle => {println!("process_task stopped"); shutdown.start()},
    }
    // Wait till shutdown procedures is finished
    let _ = shutdown.complete().await;
    println!("Exiting main...")
}
