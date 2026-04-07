use bytes::Bytes;
use reqwest::{Client, ClientBuilder, StatusCode, Url, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

/// Validated stream identifier used in HTTP protocol requests.
///
/// A `Stream` wraps a non-empty string representing a DSH data stream.
/// It is used together with [`Topic`] to form the path for retained-message
/// operations (e.g. `GET /data/v0/single/tt/<stream>/<topic>`).
///
/// Construct via [`Stream::new`], [`TryFrom<&str>`] or [`TryFrom<String>`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stream(String);

impl Stream {
    /// Create a `Stream` from anything convertible to `String`.
    ///
    /// Returns [`HttpError::InvalidInput`] when the value is empty or whitespace-only.
    pub fn new<S: Into<String>>(s: S) -> Result<Self, HttpError> {
        let s: String = s.into();
        if s.trim().is_empty() {
            return Err(HttpError::InvalidInput("stream must not be empty".into()));
        }
        Ok(Self(s))
    }
}

impl TryFrom<&str> for Stream {
    type Error = HttpError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Stream::new(s)
    }
}
impl TryFrom<String> for Stream {
    type Error = HttpError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Stream::new(s)
    }
}
impl AsRef<str> for Stream {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Validated topic identifier used in HTTP protocol requests.
///
/// A `Topic` wraps a non-empty string representing an MQTT-style topic.
/// Wildcard characters (`+` for single-level, `#` for multi-level) are
/// supported in [`HttpClient::multi_get`] topic filters.
///
/// Construct via [`Topic::new`], [`TryFrom<&str>`] or [`TryFrom<String>`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Topic(String);

impl Topic {
    pub fn new(string: impl Into<String>) -> Self {
        let s = string.into();
        if s.trim().is_empty() {
            Self("#".to_string()) // default to wildcard if empty
        } else {
            Self(s)
        }
    }
}

impl From<&str> for Topic {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Topic {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
impl AsRef<str> for Topic {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Response body returned by [`HttpClient::get_retained`].
///
/// The variant depends on the [`Accept`] type used in the request:
/// - [`Accept::ApplicationOctetStream`] → [`ResponseBody::Bytes`]
/// - All other accept types → [`ResponseBody::Text`]
#[derive(Debug)]
pub enum ResponseBody {
    /// UTF-8 text payload.
    Text(String),
    /// Raw binary payload.
    Bytes(Bytes),
}

/// Accepted response content types for HTTP protocol requests.
///
/// Used as the HTTP `Accept` header value to control the format of the
/// returned payload.
///
/// | Variant                    | Header value              | Supported by                                                 |
/// |----------------------------|---------------------------|--------------------------------------------------------------|
/// | `TextPlain`                | `text/plain`              | [`get_retained`](HttpClient::get_retained), [`multi_get`](HttpClient::multi_get) |
/// | `ApplicationJson`          | `application/json`        | [`get_retained`](HttpClient::get_retained)                    |
/// | `ApplicationOctetStream`   | `application/octet-stream`| [`get_retained`](HttpClient::get_retained)                    |
/// | `Base64`                   | `base64`                  | [`get_retained`](HttpClient::get_retained), [`multi_get`](HttpClient::multi_get) |
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accept {
    TextPlain,
    ApplicationJson,
    ApplicationOctetStream,
    Base64,
}
impl Accept {
    /// Value for the HTTP `Accept` header.
    pub fn header_value(self) -> &'static str {
        match self {
            Accept::TextPlain => "text/plain",
            Accept::ApplicationJson => "application/json",
            Accept::ApplicationOctetStream => "application/octet-stream",
            Accept::Base64 => "base64",
        }
    }
}

/// Content types supported for POST request payloads.
///
/// Used as the HTTP `Content-Type` header value when publishing a retained
/// message via [`HttpClient::post_retained_body`] or [`HttpClient::post_retained_file`].
///
/// | Variant                  | Header value              |
/// |--------------------------|---------------------------|
/// | `TextPlain`              | `text/plain`              |
/// | `ApplicationOctetStream` | `application/octet-stream`|
/// | `Base64`                 | `base64`                  |
///
/// # Note
/// DSH does not support `application/json` as a POST content-type.
/// To send JSON payloads, use [`ContentType::TextPlain`] and encode
/// the JSON as a UTF-8 byte string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
    Base64,
}
impl ContentType {
    /// Value for the HTTP `Content-Type` header.
    pub fn header_value(self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationOctetStream => "application/octet-stream",
            ContentType::Base64 => "base64",
        }
    }
}

/// Errors returned by [`HttpClient`] operations.
///
/// | Variant        | Cause                                                                 |
/// |----------------|-----------------------------------------------------------------------|
/// | `InvalidInput` | Caller supplied invalid arguments (empty stream/topic, bad QoS, …)    |
/// | `Request`      | Network or TLS error from the underlying `reqwest` client             |
/// | `Status`       | Server returned a non-success HTTP status code                        |
/// | `Json`         | Failed to deserialise a JSON response (e.g. in [`HttpClient::multi_get`]) |
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("HTTP status {code}: {body}")]
    Status { code: StatusCode, body: String },
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

/// Request payload for multi-get.
#[derive(Serialize)]
struct MultiGetRequest<'a> {
    #[serde(rename = "content-type")]
    content_type: &'a str,
    #[serde(rename = "topic-filters")]
    topic_filters: Vec<String>,
}

/// A single item returned by [`HttpClient::multi_get`].
///
/// The DSH multi-get endpoint returns a JSON array of objects, each
/// containing a fully-qualified `topic` (e.g. `/tt/<stream>/<topic>`)
/// and the retained `payload` encoded according to the requested
/// [`Accept`] type.
#[derive(Debug, Deserialize)]
pub struct MultiGetItem {
    /// Fully-qualified topic path (e.g. `/tt/my-stream/sensors/temp/room1`).
    pub topic: String,
    /// Retained payload as a string (text or base64-encoded, depending on the request).
    pub payload: String,
}

/// HTTP client for the DSH Messaging API HTTP Protocol Adapter.
///
/// `HttpClient` wraps a [`reqwest::Client`] together with a base URL and provides
/// typed methods for every retained-message operation the DSH HTTP protocol
/// adapter exposes:
///
/// - [`get_retained`](HttpClient::get_retained) — fetch a single retained message
/// - [`post_retained_body`](HttpClient::post_retained_body) — publish a retained message from bytes
/// - [`post_retained_file`](HttpClient::post_retained_file) — publish a retained message from a file
/// - [`delete_retained`](HttpClient::delete_retained) — remove a retained message
/// - [`multi_get`](HttpClient::multi_get) — fetch multiple messages using wildcard topic filters
///
/// # Construction
/// Use [`HttpClient::builder`] to create a client with sensible defaults, or
/// [`HttpClient::with_client`] to wrap an existing `reqwest::Client`.
///
/// ```no_run
/// use dsh_sdk::protocol_adapters::http_protocol::HttpClient;
/// use std::time::Duration;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Via builder (creates an HTTPS-only reqwest client internally)
/// let client = HttpClient::builder("https://protocol-adapter.example.com")
///     .timeout(Duration::from_secs(15))
///     .build()?;
///
/// // Or wrap an existing reqwest::Client
/// let custom = reqwest::Client::builder()
///     .timeout(Duration::from_secs(30))
///     .build()?;
/// let client = HttpClient::with_client("https://protocol-adapter.example.com", custom)?;
/// # Ok(())
/// # }
/// ```
///
/// # Authentication
/// Every request requires a valid MQTT / Data Access Token passed as the `token`
/// parameter. The token is sent as a `Bearer` authorization header.
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: String,
}

/// Builder for constructing an [`HttpClient`].
///
/// Created via [`HttpClient::builder`] or [`HttpClientBuilder::new`].
/// The builder lets you configure:
///
/// | Setting   | Default   | Description                                 |
/// |-----------|-----------|---------------------------------------------|
/// | `timeout` | 10 seconds| Request timeout for the underlying HTTP client |
///
/// Call [`build`](HttpClientBuilder::build) to produce a ready-to-use
/// [`HttpClient`] with an HTTPS-only `reqwest::Client`.
#[derive(Debug)]
pub struct HttpClientBuilder {
    base_url: String,
    timeout: Duration,
}

impl HttpClientBuilder {
    /// Create a new builder with the given `base_url`.
    ///
    /// The URL is stored as-is and validated only when [`build`](HttpClientBuilder::build)
    /// is called. The default request timeout is 10 seconds.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            timeout: Duration::from_secs(10),
        }
    }
    /// Override the request timeout (default: 10 seconds).
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }
    /// Consume the builder and create an [`HttpClient`].
    ///
    /// Parses and normalizes the base URL (strips the path component so that
    /// API routes are always appended to the scheme + host) and builds an
    /// HTTPS-only `reqwest::Client`.
    ///
    /// Returns [`HttpError::InvalidInput`] if the base URL cannot be parsed.
    pub fn build(self) -> Result<HttpClient, HttpError> {
        let mut parsed = Url::parse(&self.base_url)
            .map_err(|e| HttpError::InvalidInput(format!("invalid base_url: {e}")))?;
        parsed.set_path(""); // always normalize
        let base_url = parsed.as_str().trim_end_matches('/').to_string();
        let client = ClientBuilder::new()
            .https_only(true)
            .timeout(self.timeout)
            .build()?;
        Ok(HttpClient { client, base_url })
    }
}

impl HttpClient {
    /// Start building an [`HttpClient`] from a base URL.
    ///
    /// Shorthand for [`HttpClientBuilder::new`].
    pub fn builder(base_url: &str) -> HttpClientBuilder {
        HttpClientBuilder::new(base_url)
    }

    /// Create an [`HttpClient`] using an externally provided [`reqwest::Client`].
    ///
    /// Use this when you need full control over the HTTP client configuration
    /// (e.g. custom TLS settings, proxy, connection pool). The `base_url` path
    /// component is stripped, just like in the builder.
    pub fn with_client(base_url: &str, client: reqwest::Client) -> Result<Self, HttpError> {
        let mut parsed = Url::parse(base_url)
            .map_err(|e| HttpError::InvalidInput(format!("invalid base_url: {e}")))?;
        parsed.set_path("");
        let base_url = parsed.as_str().trim_end_matches('/').to_string();
        Ok(Self { client, base_url })
    }

    /// Fetch a single retained message for the given stream and topic.
    ///
    /// Sends `GET /data/v0/single/tt/<stream>/<topic>` with a `Bearer` token.
    ///
    /// The [`Accept`] parameter controls the response format:
    /// - [`Accept::ApplicationOctetStream`] → [`ResponseBody::Bytes`]
    /// - All other variants → [`ResponseBody::Text`]
    ///
    /// # Errors
    /// - [`HttpError::Request`] on network / TLS failure.
    /// - [`HttpError::Status`] when the server returns a non-success status code
    ///   (e.g. `404` if no retained message exists).
    pub async fn get_retained(
        &self,
        stream: &Stream,
        topic: &Topic,
        accept: Accept,
        token: &str,
    ) -> Result<ResponseBody, HttpError> {
        let url = format!(
            "{}/data/v0/single/tt/{}/{}",
            self.base_url,
            stream.as_ref(),
            topic.as_ref()
        );
        let resp = self
            .client
            .get(url)
            .header(header::ACCEPT, accept.header_value())
            .bearer_auth(token)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(HttpError::Status {
                code: status,
                body: resp.text().await.unwrap_or("<failed to read body>".into()),
            });
        }
        let body = match accept {
            Accept::ApplicationOctetStream => ResponseBody::Bytes(resp.bytes().await?),
            _ => ResponseBody::Text(resp.text().await?),
        };
        Ok(body)
    }

    /// Publish a retained message for the given stream and topic.
    ///
    /// Sends `POST /data/v0/single/tt/<stream>/<topic>` with a `Bearer` token.
    ///
    /// ## Parameters
    /// | Parameter      | Type                   | Default   | Description                                                        |
    /// |----------------|------------------------|-----------|--------------------------------------------------------------------|
    /// | `stream`       | `&Stream`              | _required_| Target data stream                                                 |
    /// | `topic`        | `&Topic`               | _required_| Target topic within the stream                                     |
    /// | `content_type` | [`ContentType`]        | _required_| Content-type header for the payload                                |
    /// | `token`        | `&str`                 | _required_| Data Access Token (sent as `Bearer`)                               |
    /// | `payload`      | `impl Into<Bytes>`     | _required_| Message body                                                       |
    /// | `qos`          | `Option<&str>`         | `"1"`     | Quality of Service — `"0"` (at most once) or `"1"` (at least once)  |
    /// | `retained`     | `Option<&str>`         | `"true"`  | Whether the message should be retained — `"true"` or `"false"`      |
    ///
    /// ## Quality of Service (QoS)
    /// - **0 — At most once:** best-effort delivery; the recipient does not acknowledge receipt.
    /// - **1 — At least once:** the sender stores the message until the receiver acknowledges it.
    ///   A message may be delivered more than once.
    ///
    /// ## Retained messages
    /// When `retained` is `"true"`, any client that later subscribes to this MQTT topic will
    /// immediately receive the retained message.
    ///
    /// # Errors
    /// - [`HttpError::InvalidInput`] if `qos` is not `"0"` or `"1"`.
    /// - [`HttpError::InvalidInput`] if the payload exceeds 128 KB.
    /// - [`HttpError::Request`] on network / TLS failure.
    /// - [`HttpError::Status`] when the server returns a non-success status code.
    pub async fn post_retained_body(
        &self,
        stream: &Stream,
        topic: &Topic,
        content_type: ContentType,
        token: &str,
        payload: impl Into<Bytes>,
        qos: Option<&str>,
        retained: Option<&str>,
    ) -> Result<(), HttpError> {
        let payload: Bytes = payload.into();
        if payload.len() > 128 * 1024 {
            return Err(HttpError::InvalidInput(format!(
                "payload is {} bytes, exceeding the 128 KB limit",
                payload.len()
            )));
        }
        let qos_value = qos.unwrap_or("1");
        let retained_value = retained.unwrap_or("true");
        if qos_value != "1" && qos_value != "0" {
            return Err(HttpError::InvalidInput(format!(
                "Invalid QoS value: {}. Only '1' or '0' are allowed.",
                qos_value
            )));
        }
        let url = format!(
            "{}/data/v0/single/tt/{}/{}?qos={}&retained={}",
            self.base_url,
            stream.as_ref(),
            topic.as_ref(),
            qos_value,
            retained_value
        );
        let resp = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, content_type.header_value())
            .bearer_auth(token)
            .body(payload)
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(HttpError::Status {
                code: resp.status(),
                body: resp.text().await.unwrap_or("<failed to read body>".into()),
            });
        }
        Ok(())
    }

    /// Publish a retained message by reading the payload from a local file.
    ///
    /// Reads the file at `path` into memory and delegates to
    /// [`post_retained_body`](HttpClient::post_retained_body) with the default
    /// QoS (`1`) and retained (`true`) settings.
    ///
    /// # Note
    /// Intended for text files or images. The payload must not exceed 128 KB
    /// (enforced by [`post_retained_body`](HttpClient::post_retained_body)).
    ///
    /// # Errors
    /// - [`HttpError::InvalidInput`] if the file cannot be read.
    /// - Any error produced by [`post_retained_body`](HttpClient::post_retained_body).
    pub async fn post_retained_file(
        &self,
        stream: &Stream,
        topic: &Topic,
        content_type: ContentType,
        token: &str,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), HttpError> {
        let payload = std::fs::read(&path).map_err(|e| {
            HttpError::InvalidInput(format!(
                "failed to read file `{}`: {e}",
                path.as_ref().display()
            ))
        })?;
        self.post_retained_body(stream, topic, content_type, token, payload, None, None)
            .await
    }

    /// Delete the retained message for the given stream and topic.
    ///
    /// Sends `DELETE /data/v0/single/tt/<stream>/<topic>` with a `Bearer` token.
    ///
    /// # Errors
    /// - [`HttpError::Request`] on network / TLS failure.
    /// - [`HttpError::Status`] when the server returns a non-success status code.
    pub async fn delete_retained(
        &self,
        stream: &Stream,
        topic: &Topic,
        token: &str,
    ) -> Result<(), HttpError> {
        let url = format!(
            "{}/data/v0/single/tt/{}/{}",
            self.base_url,
            stream.as_ref(),
            topic.as_ref()
        );
        let resp = self.client.delete(url).bearer_auth(token).send().await?;
        if !resp.status().is_success() {
            return Err(HttpError::Status {
                code: resp.status(),
                body: resp.text().await.unwrap_or("<failed to read body>".into()),
            });
        }
        Ok(())
    }

    /// Fetch retained messages for one or more topic filters in a single request.
    ///
    /// Sends `POST /data/v0/multi` with a JSON body containing the topic filters.
    /// Supports MQTT-style wildcards in the topic filters:
    /// - `+` — matches exactly one topic level (e.g. `sensors/temp/+`)
    /// - `#` — matches all levels under the given prefix (e.g. `sensors/#`)
    ///
    /// # ⚠ Partitioner prerequisite
    /// The target stream **must** use a **topic-level partitioner** for wildcard
    /// filters to work correctly. With other partitioning strategies, related
    /// topics may be spread across different partitions. Because this method
    /// searches only a single partition, topics on other partitions will **not**
    /// be returned — resulting in incomplete or empty results even when the
    /// retained messages exist.
    ///
    /// ## Supported Accept types
    /// Only [`Accept::TextPlain`] and [`Accept::Base64`] are supported.
    /// Other variants return [`HttpError::InvalidInput`].
    ///
    /// ## Return value
    /// Returns a `Vec<MultiGetItem>`, where each item contains the fully-qualified
    /// topic path and the retained payload. An empty `Vec` is returned when no
    /// retained messages match the filters.
    ///
    /// # Errors
    /// - [`HttpError::InvalidInput`] if `accept` is not `TextPlain` or `Base64`.
    /// - [`HttpError::Request`] on network / TLS failure.
    /// - [`HttpError::Status`] when the server returns a non-success status code.
    /// - [`HttpError::Json`] if the response cannot be parsed.
    pub async fn multi_get(
        &self,
        stream: &Stream,
        topics: &[Topic],
        accept: Accept,
        token: &str,
    ) -> Result<Vec<MultiGetItem>, HttpError> {
        // Inner content-type is restricted.
        let inner_ct = match accept {
            Accept::TextPlain => "text/plain",
            Accept::Base64 => "base64",
            _ => {
                return Err(HttpError::InvalidInput(
                    "multi-get only supports Accept::TextPlain or Accept::Base64".into(),
                ));
            }
        };

        let filters: Vec<String> = topics
            .iter()
            .map(|t| format!("/tt/{}/{}", stream.as_ref(), t.as_ref()))
            .collect();

        let req = MultiGetRequest {
            content_type: inner_ct,
            topic_filters: filters,
        };

        let url = format!(
            "{}/data/v0/multi",
            self.base_url
        );
        let resp = self
            .client
            .post(url)
            .json(&req)
            .bearer_auth(token)
            .send()
            .await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(HttpError::Status {
                code: status,
                body: resp.text().await.unwrap_or("<failed to read body>".into()),
            });
        }

        let text = resp.text().await?;
        let items: Vec<MultiGetItem> = serde_json::from_str(&text)?;
        Ok(items)
    }
}

#[cfg(all(test, feature = "http-protocol-adapter"))]
mod tests {
    use super::*;
    use mockito::{Matcher, Server};

    /// Verifies:
    /// - Correct POST /data/v0/multi with JSON body having /tt/<stream>/<topic-filter>
    /// - Wildcard '+' returns multiple items and is parsed by the client
    #[tokio::test]
    async fn multi_get_plus_returns_multiple_items() {
        let mut server = Server::new_async().await;

        let expected_subset = serde_json::json!({
            "topic-filters": ["/tt/greenbox-test/sensors/temp/+"]
        });

        let mock = server
            .mock("POST", "/data/v0/multi")
            .match_header("authorization", "Bearer TOKEN")
            .match_header("content-type", "application/json")
            .match_body(Matcher::PartialJson(expected_subset))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                [
                  {"topic": "/tt/greenbox-test/sensors/temp/room1", "payload": "21.5"},
                  {"topic": "/tt/greenbox-test/sensors/temp/room2", "payload": "22.1"}
                ]
            "#,
            )
            .create_async()
            .await;

        let client =
            HttpClient::with_client(server.url().as_str(), reqwest::Client::new()).unwrap();

        let stream = Stream::try_from("greenbox-test").unwrap();
        let filter = Topic::try_from("sensors/temp/+").unwrap();

        let items = client
            .multi_get(&stream, &[filter], Accept::TextPlain, "TOKEN")
            .await
            .unwrap();

        println!("--- DEBUG: items returned = {} ---", items.len());
        for (idx, it) in items.iter().enumerate() {
            println!("[{}] {} => {}", idx, it.topic, it.payload);
        }
        println!("----------------------------------");

        mock.assert_async().await;
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].topic, "/tt/greenbox-test/sensors/temp/room1");
        assert_eq!(items[0].payload, "21.5");
        assert_eq!(items[1].topic, "/tt/greenbox-test/sensors/temp/room2");
        assert_eq!(items[1].payload, "22.1");
    }

    /// Verifies that `HttpClientBuilder::build` returns `HttpError::InvalidInput`
    /// when given an unparseable URL.
    #[test]
    fn builder_rejects_invalid_base_url() {
        assert!(matches!(
            HttpClientBuilder::new("not-a-valid-url").build(),
            Err(HttpError::InvalidInput(_))
        ));
    }

    /// Verifies that `HttpClientBuilder::build` succeeds with a valid URL.
    #[test]
    fn builder_accepts_valid_base_url() {
        assert!(matches!(
            HttpClientBuilder::new("https://protocol-adapter.example.com").build(),
            Ok(_)
        ));
    }

    #[test]
    fn with_client_rejects_invalid_url() {
        assert!(matches!(
            HttpClient::with_client("not-a-url", reqwest::Client::new()),
            Err(HttpError::InvalidInput(_))
        ));
    }

    #[test]
    fn with_client_accepts_valid_url() {
        assert!(matches!(
            HttpClient::with_client("https://example.com", reqwest::Client::new()),
            Ok(_)
        ));
    }

    #[test]
    fn stream_rejects_empty() {
        assert!(matches!(
            Stream::new(""),
            Err(HttpError::InvalidInput(_))
        ));
    }

    #[test]
    fn stream_rejects_whitespace_only() {
        assert!(matches!(
            Stream::new("   "),
            Err(HttpError::InvalidInput(_))
        ));
    }

    #[test]
    fn stream_accepts_valid_name() {
        let s = Stream::new("my-stream").unwrap();
        assert_eq!(s.as_ref(), "my-stream");
    }

    #[test]
    fn stream_try_from_str() {
        assert!(Stream::try_from("ok").is_ok());
        assert!(Stream::try_from("").is_err());
    }

    #[test]
    fn stream_try_from_string() {
        assert!(Stream::try_from("ok".to_string()).is_ok());
        assert!(Stream::try_from(String::new()).is_err());
    }

    #[test]
    fn topic_defaults_to_wildcard_when_empty() {
        assert_eq!(Topic::new("").as_ref(), "#");
    }

    #[test]
    fn topic_defaults_to_wildcard_when_whitespace() {
        assert_eq!(Topic::new("   ").as_ref(), "#");
    }

    #[test]
    fn topic_preserves_value() {
        assert_eq!(Topic::new("sensors/temp").as_ref(), "sensors/temp");
    }

    #[test]
    fn topic_from_str() {
        let t: Topic = "foo".into();
        assert_eq!(t.as_ref(), "foo");
    }

    #[test]
    fn topic_from_string() {
        let t: Topic = String::from("bar").into();
        assert_eq!(t.as_ref(), "bar");
    }

    #[test]
    fn accept_header_values() {
        assert_eq!(Accept::TextPlain.header_value(), "text/plain");
        assert_eq!(Accept::ApplicationJson.header_value(), "application/json");
        assert_eq!(
            Accept::ApplicationOctetStream.header_value(),
            "application/octet-stream"
        );
        assert_eq!(Accept::Base64.header_value(), "base64");
    }

    #[test]
    fn content_type_header_values() {
        assert_eq!(ContentType::TextPlain.header_value(), "text/plain");
        assert_eq!(
            ContentType::ApplicationOctetStream.header_value(),
            "application/octet-stream"
        );
        assert_eq!(ContentType::Base64.header_value(), "base64");
    }

    // ── QoS validation ───────────────────────────────────────────────

    /// Invalid QoS values are rejected before any network I/O.
    #[tokio::test]
    async fn post_retained_body_rejects_invalid_qos() {
        let client =
            HttpClient::with_client("http://localhost", reqwest::Client::new()).unwrap();
        let stream = Stream::new("s").unwrap();
        let topic = Topic::new("t");

        for bad in &["2", "1 ", "01", "abc", ""] {
            let result = client
                .post_retained_body(
                    &stream,
                    &topic,
                    ContentType::TextPlain,
                    "tok",
                    b"hello".as_ref(),
                    Some(bad),
                    None,
                )
                .await;
            assert!(
                matches!(result, Err(HttpError::InvalidInput(_))),
                "expected InvalidInput for qos={bad:?}, got {result:?}"
            );
        }
    }

    /// QoS values "0" and "1" pass validation and reach the HTTP layer.
    #[tokio::test]
    async fn post_retained_body_accepts_valid_qos() {
        let mut server = Server::new_async().await;
        let stream = Stream::new("s").unwrap();
        let topic = Topic::new("t");

        for qos in &["0", "1"] {
            let mock = server
                .mock("POST", "/data/v0/single/tt/s/t")
                .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("qos".into(), qos.to_string()),
                    Matcher::UrlEncoded("retained".into(), "true".into()),
                ]))
                .with_status(200)
                .create_async()
                .await;

            let client =
                HttpClient::with_client(server.url().as_str(), reqwest::Client::new()).unwrap();
            client
                .post_retained_body(
                    &stream,
                    &topic,
                    ContentType::TextPlain,
                    "tok",
                    b"hello".as_ref(),
                    Some(qos),
                    None,
                )
                .await
                .unwrap_or_else(|e| panic!("qos={qos} should be accepted, got {e:?}"));

            mock.assert_async().await;
        }
    }
}
