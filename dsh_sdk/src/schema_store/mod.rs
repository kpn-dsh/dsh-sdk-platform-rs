//! Schema Store client
//!
//! This module contains the SchemaStoreClient struct which is the main entry point for interacting with the DSH Schema Registry API.
//!
//! It automatically connects to the Schema Registry API with proper certificates and uses the base URL provided by the datastreams.josn.
//!
//! When connecting via Proxy or to a local Schema Registry, you can provide the base URL yourself via the [SchemaStoreClient::new_with_base_url] function or by setting `SCHEMA_REGISTRY_HOST` variable.
//!
//! ## Example
//! ```no_run
//! use dsh_sdk::schema_store::SchemaStoreClient;
//! use dsh_sdk::schema_store::types::*;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = SchemaStoreClient::new();
//!
//! // List all subjects
//! let subjects = client.subjects().await.unwrap();
//!
//! // Get the latest version of a subjects value schema
//! let subject = client.subject(SubjectName::TopicNameStrategy{topic: "scratch.example-topic.tenant".to_string(), key: false}, SubjectVersion::Latest).await.unwrap();
//! let raw_schema = subject.schema;
//! # }
//! ```
//!
//! ## Input arguments
//! Note that for all input types [TryInto] or [Into] is implemented. This means you can use the following types as input:
//! ```
//! use dsh_sdk::schema_store::types::*;
//!
//! // From original type
//! let from_struct = SubjectName::TopicNameStrategy{topic: "scratch.example-topic.tenant".to_string(), key: false};
//!
//! // From string
//! let from_str: SubjectName = "scratch.example-topic.tenant-value".try_into().unwrap(); // Note that `-value`` is added, else it will return error as it is not a valid SubjectName
//! assert_eq!(from_str, from_struct);
//!
//! // From tuple
//! let from_tuple: SubjectName = ("scratch.example-topic.tenant", false).into();
//! assert_eq!(from_tuple, from_struct);
//! ```
//!
//! This means you can easily provide the input arguments from other types without converting it yourself.
//! For example:
//! ```no_run
//! use dsh_sdk::schema_store::SchemaStoreClient;
//! use dsh_sdk::schema_store::types::*;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let client = SchemaStoreClient::new();
//!
//! let raw_schema = r#"{ "type": "record", "name": "User", "fields": [ { "name": "name", "type": "string" } ] }"#;
//! client.subject_add_schema("scratch.example-topic.tenant-value", raw_schema).await.unwrap(); // Returns error if schema is not valid
//! # }
//! ```

mod api;
mod client;
mod error;
mod request;
pub mod types;

#[doc(inline)]
pub use client::SchemaStoreClient;
#[doc(inline)]
pub use error::SchemaStoreError;

type Result<T> = std::result::Result<T, SchemaStoreError>;
