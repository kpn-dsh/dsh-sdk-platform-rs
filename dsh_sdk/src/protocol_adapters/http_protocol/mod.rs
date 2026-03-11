//! HTTP protocol adapter (v2).
//!
//! This module exposes all public types that users need
//! when working with the HTTP Protocol Adapter.

pub mod client;

// Re-export all public API types from client.rs.
// This makes them available as:
//   http_protocol::HttpClient
//   http_protocol::Stream
//   http_protocol::Topic
//   ... etc.

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