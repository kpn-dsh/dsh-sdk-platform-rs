//! DSH HTTP Protocol Adapter
//!
//! This module provides an HTTP client for publishing, retrieving, and deleting
//! retained messages on the DSH platform via the Messaging API. The [`HttpClient`]
//! is at the core of this module, offering methods for single-message operations
//! (GET, POST, DELETE) as well as multi-get with wildcard topic filters.
//!
//! # Example
//! ```no_run
//! use dsh_sdk::protocol_adapters::http_protocol::{
//!     HttpClient, Stream, Topic, Accept, ContentType, ResponseBody,
//! };
//! use std::time::Duration;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let base_url = "https://protocol-adapter.example.com";
//! let token = "my-data-access-token";
//!
//! // Build an HTTP client targeting the platform base URL.
//! let client = HttpClient::builder(base_url)?
//!     .timeout(Duration::from_secs(10))
//!     .build()?;
//!
//! let stream = Stream::try_from("my-stream")?;
//! let topic  = Topic::try_from("my/topic")?;
//!
//! // POST a retained message.
//! client.post_retained_body(
//!     &stream, &topic, ContentType::TextPlain, token,
//!     b"hello world".to_vec(), None, None,
//! ).await?;
//!
//! // GET the retained message back.
//! let body = client.get_retained(&stream, &topic, Accept::TextPlain, token).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Available operations
//! - [`HttpClient::get_retained`] — fetch a single retained message
//! - [`HttpClient::post_retained_body`] — publish a retained message from bytes
//! - [`HttpClient::post_retained_file`] — publish a retained message from a file
//! - [`HttpClient::delete_retained`] — remove a retained message
//! - [`HttpClient::multi_get`] — fetch multiple messages using wildcard topic filters
//!
//! # ⚠ Partitioner prerequisite for wildcard multi-get
//! The stream you target **must** use a **topic-level partitioner** for wildcard
//! topic filters (`+`, `#`) in [`HttpClient::multi_get`] to work correctly.
//!
//! With other partitioning strategies, topics that share a common prefix may end
//! up on different partitions. Because `multi_get` searches only a single
//! partition, matching topics on other partitions will **not** be returned,
//! leading to incomplete or empty results even though the retained messages exist.
//!
//! Make sure the stream is configured with a topic-level partitioner before
//! relying on wildcard queries.

pub mod client;

pub use client::{
    HttpClient,
    HttpClientBuilder,
    Stream,
    Topic,
    Accept,
    ContentType,
    ResponseBody,
    MultiGetItem,
    HttpError,
};