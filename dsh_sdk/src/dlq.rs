//! # Dead Letter Queue
//! This optional module contains an implementation of pushing unprocessable/invalid messages towards a Dead Letter Queue (DLQ).
//!
//! add feature `dlq` to your Cargo.toml to enable this module
//!
//! ### NOTE:
//! This module is meant for pushing messages towards a dead/retry topic only, it does and WILL not handle any logic for retrying messages.
//! Reason is, it can differ per use case what strategy is needed to retry messages and handle the dead letters.
//!
//! It is up to the user to implement the strategy and logic for retrying messages.
//!
//! ### How it works
//! The DLQ struct can
//!
//! ## How to use
//! 1. Implement the `ErrorToDlq` trait on top your (custom) error type.
//! 2. Initialize the `Dlq` struct in your service in main.
//! 3. Get the dlq channel sender from the `Dlq` struct and use this channel to communicate with the `Dlq` struct from other threads.
//! 4. Run the `Dlq` struct in a separate tokio thread. This will run the producer that will produce towards the dead/retry topics.
//!
//! The topics are set via environment variables DLQ_DEAD_TOPIC and DLQ_RETRY_TOPIC.
//!
//! ### Example:
//! See the examples folder on github for a working example.

#[deprecated(
    since = "0.5.0",
    note = "The DLQ is moved to [crate::utils::dlq](crate::utils::dlq)"
)]
pub use crate::utils::dlq::*;
