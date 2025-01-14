//! Graceful Shutdown Module
//!
//! This module provides a handle for initiating and coordinating graceful shutdown of your
//! application (e.g., ending background tasks in a controlled manner). It listens for
//! Unix/MacOS/Windows signals (`SIGTERM`, `SIGINT`) and broadcasts a shutdown request to any
//! cloned handles. When a shutdown is requested, tasks can finalize operations before
//! exiting, ensuring a clean teardown of the service.
//!
//! The design is inspired by [Tokio’s graceful shutdown approach](https://tokio.rs/tokio/topics/shutdown).
//!
//! # Key Components
//! - [`Shutdown`] struct: The main handle that tasks can clone to receive or initiate shutdown.
//! - **Signal Handling**: [`Shutdown::signal_listener`] blocks on system signals and triggers a shutdown.
//! - **Manual Trigger**: [`Shutdown::start`] can be called to programmatically start shutdown.
//! - **Completion Wait**: [`Shutdown::complete`] ensures that all tasks have finished before the main thread exits.
//!
//! # Table of Methods
//! | **Method**                  | **Description**                                                                                                    |
//! |---------------------------- |--------------------------------------------------------------------------------------------------------------------|
//! | [`Shutdown::new`]          | Creates a fresh [`Shutdown`] handle, along with a channel to track completion.                                      |
//! | [`Shutdown::clone`]        | Clone a [`Shutdown`] handle which is linked to the original handle.                                                |
//! | [`Shutdown::start`]        | Signals all clones that a shutdown is in progress, causing each to break out of their loops.                       |
//! | [`Shutdown::recv`]         | Awaitable method for a cloned handle to detect when a shutdown has started.                                        |
//! | [`Shutdown::signal_listener`] | Waits for `SIGTERM`/`SIGINT`, then calls [`start`](Shutdown::start) automatically to notify the other handles.  |
//! | [`Shutdown::complete`]     | Waits for all handles are finished before returning, ensuring a graceful final exit.                               |
//!
//! # Usage Example
//! ```no_run
//! use dsh_sdk::utils::graceful_shutdown::Shutdown;
//! use tokio::time::{sleep, Duration};
//!
//! // A background task that runs until shutdown is requested.
//! async fn process_task(shutdown: Shutdown) {
//!     loop {
//!         tokio::select! {
//!             _ = sleep(Duration::from_secs(1)) => {
//!                 // Perform background work (e.g., read from Kafka, handle jobs, etc.)
//!                 println!("Still processing the task...");
//!             },
//!             _ = shutdown.recv() => {
//!                 // A shutdown signal was received; finalize or clean up as needed.
//!                 println!("Gracefully exiting process_task");
//!                 break;
//!             },
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create the primary shutdown handle
//!     let shutdown = Shutdown::new();
//!
//!     // Clone the handle for use in background tasks
//!     let cloned_shutdown = shutdown.clone();
//!     let process_task_handle = tokio::spawn(async move {
//!         process_task(cloned_shutdown).await;
//!     });
//!
//!     // Concurrently wait for OS signals OR for the background task to exit
//!     tokio::select! {
//!         // If a signal (SIGINT or SIGTERM) is received, initiate shutdown
//!         _ = shutdown.signal_listener() => println!("Exit signal received!"),
//!
//!         // If the background task completes on its own, start the shutdown
//!         _ = process_task_handle => {
//!             println!("process_task stopped");
//!             shutdown.start();
//!         },
//!     }
//!
//!     // Wait for all tasks to acknowledge the shutdown and finish
//!     shutdown.complete().await;
//!     println!("All tasks have completed. Exiting main...");
//! }
//! ```

use log::{info, warn};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

/// A handle that facilitates graceful shutdown of the application or individual tasks.
///
/// Cloning this handle allows tasks to listen for shutdown signals (internal or
/// from the OS). The original handle can trigger the shutdown and subsequently
/// await the completion of all other handles through [`Shutdown::complete`].
///
/// # Usage
/// 1. **Create** a primary handle with [`Shutdown::new`].  
/// 2. **Clone** it to each task that needs to respond to a shutdown signal.  
/// 3. **Optionally** call [`Shutdown::signal_listener`] in your main or toplevel to wait for OS signals (`SIGTERM`, `SIGINT`).  
/// 4. **Call** [`Shutdown::start`] manually if you’d like to trigger a shutdown yourself (e.g., error condition).  
/// 5. **Await** [`Shutdown::complete`] to ensure all tasks are finished.  
#[derive(Debug)]
pub struct Shutdown {
    cancel_token: CancellationToken,
    shutdown_complete_tx: mpsc::Sender<()>,
    shutdown_complete_rx: Option<mpsc::Receiver<()>>,
}

impl Shutdown {
    /// Creates a new shutdown handle and a completion channel.
    ///
    /// # Details
    /// - The returned handle can be cloned for other tasks.
    /// - The original handle retains a `Receiver` so it can wait for the final
    ///   signal indicating all tasks have ended (`complete`).
    ///
    /// # Note
    /// Ensure that you only keep the original handle in your main function or 
    /// manager. 
    pub fn new() -> Self {
        let cancel_token = CancellationToken::new();
        let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
        Self {
            cancel_token,
            shutdown_complete_tx,
            shutdown_complete_rx: Some(shutdown_complete_rx),
        }
    }

    /// Initiates the shutdown sequence, notifying all clone holders to stop.
    ///
    /// This effectively cancels the [`CancellationToken`], causing any tasks
    /// awaiting [`recv`](Self::recv) to return immediately. With this, all the
    /// other handles know when to gracefully shut down.
    ///
    /// # Example
    /// ```
    /// # use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// let shutdown = Shutdown::new();
    /// // ... spawn tasks ...
    /// shutdown.start(); // triggers `recv` in all clones
    /// ```
    pub fn start(&self) {
        self.cancel_token.cancel();
    }

    /// Awaits a shutdown signal.
    ///
    /// If [`start`](Self::start) has already been called, this returns immediately.
    /// Otherwise, it suspends the task until the shutdown is triggered or a signal is received.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// async fn background_task(shutdown: Shutdown) {
    ///     loop {
    ///         // Do work here...
    ///         tokio::select! {
    ///             _ = shutdown.recv() => {
    ///                 // time to clean up
    ///                 break;
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn recv(&self) {
        self.cancel_token.cancelled().await;
    }

    /// Waits for an external shutdown signal (`SIGINT` or `SIGTERM`) and then calls [`start`](Self::start).
    ///
    /// ## Compatibility
    /// - **Unix**: Waits for `SIGTERM` or `SIGINT`.
    /// - **Windows**: Waits for `SIGINT` (Ctrl-C).
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// #[tokio::main]
    /// async fn main() {
    ///     let shutdown = Shutdown::new();
    ///     tokio::spawn({
    ///         let s = shutdown.clone();
    ///         async move {
    ///             // Some worker logic...
    ///             s.recv().await;
    ///             // Cleanup worker
    ///         }
    ///     });
    ///
    ///     // Main thread checks for signals
    ///     shutdown.signal_listener().await;
    ///
    ///     // All tasks are signaled to shut down
    ///     shutdown.complete().await;
    ///     println!("All done!");
    /// }
    /// ```
    pub async fn signal_listener(&self) {
        let ctrl_c_signal = tokio::signal::ctrl_c();

        #[cfg(unix)]
        let mut sigterm_signal =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();

        #[cfg(unix)]
        tokio::select! {
            _ = ctrl_c_signal => {},
            _ = sigterm_signal.recv() => {}
        }

        #[cfg(windows)]
        let _ = ctrl_c_signal.await;

        warn!("Shutdown signal received!");
        self.start();
    }

    /// Waits for all tasks to confirm they have shut down.
    ///
    /// This consumes the original [`Shutdown`] handle (the one that includes the
    /// receiver), dropping the `Sender` so that `recv` eventually returns.
    /// Useful to ensure that no tasks remain active before final exit.
    ///
    /// # Note
    /// Calling `complete` on a cloned handle is invalid, as clones can not hold
    /// the completion receiver.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// #[tokio::main]
    /// async fn main() {
    ///     let shutdown = Shutdown::new();
    ///     // spawn tasks with shutdown.clone()...
    ///     shutdown.start();
    ///     shutdown.complete().await; // blocks until all tasks have reported completion
    ///     println!("Graceful shutdown finished!");
    /// }
    /// ```
    pub async fn complete(self) {
        // Dropping the transmitter ensures that once all clones are dropped,
        // the channel closes. The last task to shut down doesn't hold a Tx,
        // so the moment they stop using it, the channel will close.
        drop(self.shutdown_complete_tx);

        // Wait for the channel to be closed (i.e., all tasks done).
        self.shutdown_complete_rx.unwrap().recv().await;
        info!("Shutdown complete!");
    }
}

impl Default for Shutdown {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Shutdown {
    /// Creates a cloned [`Shutdown`] handle that can receive and/or trigger
    /// shutdown, but does **not** hold the channel receiver for `complete`.
    ///
    /// # Example
    /// ```no_run
    /// # use dsh_sdk::utils::graceful_shutdown::Shutdown;
    /// async fn worker_task(shutdown: Shutdown) {
    ///     shutdown.recv().await;
    ///     // Cleanup...
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shutdown = Shutdown::new();
    ///     let worker_shutdown = shutdown.clone();
    ///     tokio::spawn(worker_task(worker_shutdown));
    ///     // ...
    /// }
    /// ```
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
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_shutdown_recv() {
        let shutdown = Shutdown::new();
        let shutdown_clone = shutdown.clone();
        // This task listens for shutdown:
        let task = tokio::spawn(async move {
            shutdown_clone.recv().await;
            1
        });
        // Trigger shutdown after 200ms
        tokio::spawn({
            let s = shutdown.clone();
            async move {
                tokio::time::sleep(Duration::from_millis(200)).await;
                s.start();
            }
        });
        // If no shutdown within 5s, fail
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

        // A task that waits for shutdown, then sets a flag
        tokio::spawn(async move {
            shutdown_clone.recv().await;
            tokio::time::sleep(Duration::from_millis(200)).await;
            let mut guard = check_value_clone.lock().unwrap();
            *guard = true;
        });

        // Initiate shutdown
        shutdown.start();
        // Ensure all tasks are done
        shutdown.complete().await;

        let guard = check_value.lock().unwrap();
        assert!(
            *guard,
            "Shutdown did not successfully wait for completion of tasks."
        );
    }
}
