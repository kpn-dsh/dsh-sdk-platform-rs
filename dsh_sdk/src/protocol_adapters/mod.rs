#[cfg(feature = "http-protocol-adapter")]
pub mod http_protocol;
#[cfg(feature = "kafka")]
pub mod kafka_protocol;
#[cfg(feature = "mqtt-protocol-adapter")]
pub mod mqtt_protocol;
#[cfg(feature = "protocol-token-fetcher")]
pub mod token_fetcher;

mod error;

#[cfg(feature = "protocol-token-fetcher")]
#[doc(inline)]
pub use error::ProtocolTokenError;
#[cfg(feature = "protocol-token-fetcher")]
#[doc(inline)]
pub use token_fetcher::ProtocolTokenFetcher;
