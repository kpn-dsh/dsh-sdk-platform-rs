//! # DSH
//!
//! Dsh properties struct. Create new to initialize all related components to connect to the DSH kafka clusters
//! - Contains a struct similar to datastreams.json
//! - Metadata of running container/task
//! - Certificates for Kafka and DSH
//!
//!
//! ## Local
//!
//! It is possible to connect to local kafka cluster by enabling the `local` feature.
//! This enables to read in the local_datastreams.json file from root folder and parses it into the datastream struct inside the properties struct.
//!
//! See [local](local/index.html) for more information.
//!
//! # Graceful shutdown
//!
//! To implement a graceful shutdown in your service, you can use the `Shutdown` struct. This struct has an implementation based on the best practice example of Tokio.
//!
//! This gives you the option to properly handle shutdown in your components/tasks.
//! It listens for SIGTERM requests and sends out shutdown requests to all shutdown handles.
//! 
//! See [graceful_shutdown](graceful_shutdown/index.html) for more information.
//!
//! # DLQ (Dead Letter Queue)
//!
//! `OPTIONAL feature: dlq`
//!
//! This is an experimental feature and is not yet finalized.
//!
//! This implementation only includes pushing messages towards a kafka topic. (Dead or Retry topic)
//! ### NOTE:
//! This implementation does not (and will not) handle any other DLQ related tasks like:
//!     - Retrying messages
//!     - Handling messages in DLQ
//!     - Monitor the DLQ
//! Above tasks should be handled by a seperate component set up by the user, as these tasks are use case specific and can handle different strategies.
//!
//!
//! The DLQ is implemented by running the `Dlq` struct to push messages towards the DLQ topics.
//! The `ErrorToDlq` trait can be implemented on your defined errors, to be able to send messages towards the DLQ Struct.

pub mod dsh;

#[cfg(feature = "dlq")]
pub mod dlq;

mod error;
#[cfg(feature = "graceful_shutdown")]
pub mod graceful_shutdown;
