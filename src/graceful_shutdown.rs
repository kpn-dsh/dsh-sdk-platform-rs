//! Graceful shutdown for tokio tasks.
//!
//! This module provides a shutdown handle for graceful shutdown of (tokio tasks within) your service.
//! It listens for SIGTERM requests and sends out shutdown requests to all shutdown handles.
//!
//! It creates a clonable object which can be used to send shutdown request to all tasks.
//! Based on this request you are able to handle your shutdown procedure.
//!
//! This appproach is based on Tokio's graceful shutdown example:
//! <https://tokio.rs/tokio/topics/shutdown>
//!
//! # Example:
//! 
//! ```no_run
//! use dsh_sdk::graceful_shutdown::Shutdown;
//!
//! // your process task
//! async fn process_task(shutdown: Shutdown) {
//!     loop {
//!         tokio::select! {
//!             _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
//!                 // Do something here, e.g. consume messages from Kafka
//!                 println!("Still processing the task")
//!             },
//!             _ = shutdown.recv() => {
//!                 // shutdown request received, include your shutdown procedure here e.g. close db connection
//!                 println!("Gracefully exiting process_task");
//!                 break;
//!             },
//!         }
//!     }
//! }
//!
//! #[tokio::main]  
//! async fn main() {
//!     // Create shutdown handle
//!     let shutdown = dsh_sdk::graceful_shutdown::Shutdown::new();
//!     // Create your process task with a cloned shutdown handle
//!     let process_task = process_task(shutdown.clone());
//!     // Spawn your process task in a tokio runtime
//!     let process_task_handle = tokio::spawn(async move {
//!         process_task.await;
//!     });
//!     
//!     // Listen for shutdown request or if process task stopped
//!     // If your process stops, start shutdown procedure to stop other tasks (if any)
//!     tokio::select! {
//!         _ = shutdown.signal_listener() => println!("Exit signal received!"),
//!         _ = process_task_handle => {println!("process_task stopped"); shutdown.start()},
//!     }
//!     // Wait till shutdown procedures is finished
//!     let _ = shutdown.complete().await;
//!     println!("Exiting main...")
//! }
//! ```

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// Shutdown handle to interact on SIGTERM of DSH for a graceful shutdown.
///
/// Use original to wait for shutdown complete.
/// Use clone to send shutdown request to all shutdown handles.
///
/// see [dsh_sdk::graceful_shutdown](index.html) for full implementation example.
pub struct Shutdown {
    cancel_token: CancellationToken,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: Option<mpsc::Receiver<()>>,
}

impl Shutdown {
    /// Create new shutdown handle.
    /// Returns shutdown handle and shutdown complete receiver.
    /// Shutdown complete receiver is used to wait for all tasks to finish.
    ///
    /// NOTE: Make sure to clone shutdown handles to use it in other components/tasks.
    /// Use orignal in main and receive shutdown complete.
    pub fn new() -> Self {
        let cancel_token = CancellationToken::new();
        let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
        Self {
            cancel_token,
            shutdown_complete_tx,
            shutdown_complete_rx: Some(shutdown_complete_rx),
        }
    }

    /// Send out internal shutdown request to all Shutdown handles, so they can start their shutdown procedure.
    pub fn start(&self) {
        self.cancel_token.cancel();
    }

    /// Listen to internal shutdown request.
    /// Based on this you can start shutdown procedure in your component/task.
    pub async fn recv(&self) {
        self.cancel_token.cancelled().await;
    }

    /// Listen for external shutdown request coming from DSH (SIGTERM) or CTRL-C/SIGINT and start shutdown procedure.
    ///
    /// Compatible with Unix (SIGINT and SIGTERM) and Windows (SIGINT).
    pub async fn signal_listener(&self) {
        let ctrl_c_signal = tokio::signal::ctrl_c();
        #[cfg(unix)]
        let mut sigterm_signal =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
        #[cfg(unix)]
        tokio::select! {
            _ = ctrl_c_signal => println!("Ctrl-C received"),
            _ = sigterm_signal.recv() => println!("SIGTERM received"),
        }
        #[cfg(windows)]
        let _ = ctrl_c_signal.await;

        self.start();
    }

    /// This function can only be called by the original shutdown handle.
    ///
    /// Check if all tasks are finished and shutdown complete.
    /// This function should be awaited after all tasks are spawned.
    pub async fn complete(self) {
        // drop original shutdown_complete_tx, else it would await forever
        drop(self.shutdown_complete_tx);
        self.shutdown_complete_rx.unwrap().recv().await;
        println!("Shutdown complete!")
    }
}

impl Default for Shutdown {
    fn default() -> Self {
        Self::new()
    }
}


impl std::clone::Clone for Shutdown {
    /// Clone shutdown handle.
    ///
    /// Use this handle in your components/tasks.
    fn clone(&self) -> Self {
        Self {
            cancel_token: self.cancel_token.clone(),
            shutdown_complete_tx: self.shutdown_complete_tx.clone(),
            shutdown_complete_rx: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_shutdown_recv() {
        let shutdown = Shutdown::new();
        let shutdown_clone = shutdown.clone();
        // receive shutdown task
        let task = tokio::spawn(async move {
            shutdown_clone.recv().await;
            1
        });
        // start shutdown task after 200 ms
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            shutdown.start();
        });
        // if shutdown is not received within 5 seconds, fail test
        let check_value = tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(5)) => panic!("Shutdown not received within 5 seconds"),
            v = task => v.unwrap(),
        };
        assert_eq!(check_value, 1);
    }

    #[tokio::test]
    async fn test_shutdown_wait_for_complete() {
        let shutdown = Shutdown::new();
        let shutdown_clone = shutdown.clone();
        let check_value: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let check_value_clone = Arc::clone(&check_value);
        // receive shutdown task
        tokio::spawn(async move {
            shutdown_clone.recv().await;
            tokio::time::sleep(Duration::from_millis(200)).await;
            let mut check: std::sync::MutexGuard<'_, bool> = check_value_clone.lock().unwrap();
            *check = true;
        });
        shutdown.start();
        shutdown.complete().await;
        let check = check_value.lock().unwrap();
        assert_eq!(
            *check, true,
            "shutdown did not succesfully wait for complete"
        );
    }
}
