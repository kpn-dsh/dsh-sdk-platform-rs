use bytes::Bytes;
use reqwest::{header, Client, ClientBuilder, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

//
// ────────────────────────────────────────────────────────────────────────────────
//   TYPE MODELLING
// ────────────────────────────────────────────────────────────────────────────────
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stream(String);

impl TryFrom<&str> for Stream {
    type Error = HttpError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.trim().is_empty() {
            return Err(HttpError::InvalidInput("stream must not be empty".into()));
        }
        Ok(Self(s.to_owned()))
    }
}

impl AsRef<str> for Stream {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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
        let s = s.to_string();
        Topic::from(s)
    }
}

impl From<String> for Topic {
    fn from(s: String) -> Self {
        let v = if s.trim().is_empty() { "#".to_string() } else { s };
        Self(v)
    }
}

impl AsRef<str> for Topic {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// HTTP response body returned by GET.
#[derive(Debug)]
pub enum ResponseBody {
    Text(String),
    Bytes(Bytes),
}

//
// ────────────────────────────────────────────────────────────────────────────────
//   ACCEPT / CONTENT-TYPE
// ────────────────────────────────────────────────────────────────────────────────
//

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accept {
    TextPlain,
    ApplicationJson,
    ApplicationOctetStream,
    Base64, // DSH-specific
}

impl Accept {
    pub fn header_value(self) -> &'static str {
        match self {
            Accept::TextPlain => "text/plain",
            Accept::ApplicationJson => "application/json",
            Accept::ApplicationOctetStream => "application/octet-stream",
            Accept::Base64 => "base64", // required by DSH spec
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
    Base64,
}

impl ContentType {
    pub fn header_value(self) -> &'static str {
        match self {
            ContentType::TextPlain => "text/plain",
            ContentType::ApplicationOctetStream => "application/octet-stream",
            ContentType::Base64 => "base64", // DSH literal
        }
    }
}

//
// ────────────────────────────────────────────────────────────────────────────────
//   ERRORS
// ────────────────────────────────────────────────────────────────────────────────
//

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

//
// ────────────────────────────────────────────────────────────────────────────────
//   MULTI GET TYPES
// ────────────────────────────────────────────────────────────────────────────────
//

#[derive(Serialize)]
struct MultiGetRequest<'a> {
    #[serde(rename = "content-type")]
    content_type: &'a str,

    #[serde(rename = "topic-filters")]
    topic_filters: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct MultiGetItem {
    pub topic: String,
    pub payload: String,
}

//
// ────────────────────────────────────────────────────────────────────────────────
//   CLIENT STRUCT & BUILDER
// ────────────────────────────────────────────────────────────────────────────────
//

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    base_url: Url,
}

pub struct HttpClientBuilder {
    base_url: Url,
    timeout: Duration,
}

impl HttpClientBuilder {
    pub fn new(base_url: &str) -> Result<Self, HttpError> {
        let mut parsed = Url::parse(base_url)
            .map_err(|e| HttpError::InvalidInput(format!("invalid base_url: {e}")))?;
        parsed.set_path(""); // always normalize

        Ok(Self {
            base_url: parsed,
            timeout: Duration::from_secs(10),
        })
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Duration::from_secs(secs);
        self
    }

    pub fn build(self) -> Result<HttpClient, HttpError> {
        let client = ClientBuilder::new()
            .https_only(true)
            .timeout(self.timeout)
            .build()?;

        Ok(HttpClient {
            client,
            base_url: self.base_url,
        })
    }
}

impl HttpClient {
    pub fn builder(base_url: &str) -> Result<HttpClientBuilder, HttpError> {
        HttpClientBuilder::new(base_url)
    }

    pub fn with_client(base_url: &str, client: reqwest::Client) -> Result<Self, HttpError> {
        let mut parsed = Url::parse(base_url)
            .map_err(|e| HttpError::InvalidInput(format!("invalid base_url: {e}")))?;
        parsed.set_path(""); // ensure no unwanted trailing segments

        Ok(Self {
            client,
            base_url: parsed,
        })
    }


    //
    // ────────────────────────────────────────────────
    //   GET RETAINED
    // ────────────────────────────────────────────────
    //

    pub async fn get_retained(
        &self,
        stream: &Stream,
        topic: &Topic,
        accept: Accept,
        token: &str,
    ) -> Result<ResponseBody, HttpError> {
        ensure_non_empty("token", token)?;

        let mut url = self.base_url.clone();
        url.set_path(&format!(
            "data/v0/single/tt/{}/{}",
            stream.as_ref(),
            topic.as_ref()
        ));

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
                body: resp.text().await.unwrap_or_default(),
            });
        }

        let body = match accept {
            Accept::ApplicationOctetStream => ResponseBody::Bytes(resp.bytes().await?),
            _ => ResponseBody::Text(resp.text().await?),
        };

        Ok(body)
    }

    //
    // ────────────────────────────────────────────────
    //   POST RETAINED (body)
    // ────────────────────────────────────────────────
    //

    pub async fn post_retained_body(
        &self,
        stream: &Stream,
        topic: &Topic,
        content_type: ContentType,
        token: &str,
        body: impl AsRef<[u8]>,
    ) -> Result<(), HttpError> {
        ensure_non_empty("token", token)?;

        let mut url = self.base_url.clone();
        url.set_path(&format!(
            "data/v0/single/tt/{}/{}",
            stream.as_ref(),
            topic.as_ref()
        ));

        url.query_pairs_mut()
            .append_pair("qos", "1")
            .append_pair("retained", "true");

        let resp = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, content_type.header_value())
            .bearer_auth(token)
            .body(body.as_ref().to_vec())
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(HttpError::Status {
                code: resp.status(),
                body: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    //
    // ────────────────────────────────────────────────
    //   POST RETAINED (file)
    // ────────────────────────────────────────────────
    //

    pub async fn post_retained_file(
        &self,
        stream: &Stream,
        topic: &Topic,
        content_type: ContentType,
        token: &str,
        path: impl AsRef<std::path::Path>,
    ) -> Result<(), HttpError> {
        let bytes = std::fs::read(&path).map_err(|e| {
            HttpError::InvalidInput(format!("failed to read file `{}`: {e}", path.as_ref().display()))
        })?;

        self.post_retained_body(stream, topic, content_type, token, bytes)
            .await
    }

    //
    // ────────────────────────────────────────────────
    //   DELETE RETAINED
    // ────────────────────────────────────────────────
    //

    pub async fn delete_retained(
        &self,
        stream: &Stream,
        topic: &Topic,
        token: &str,
    ) -> Result<(), HttpError> {
        ensure_non_empty("token", token)?;

        let mut url = self.base_url.clone();
        url.set_path(&format!(
            "data/v0/single/tt/{}/{}",
            stream.as_ref(),
            topic.as_ref()
        ));

        let resp = self.client.delete(url).bearer_auth(token).send().await?;

        if !resp.status().is_success() {
            return Err(HttpError::Status {
                code: resp.status(),
                body: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    //
    // ────────────────────────────────────────────────
    //   MULTI GET
    // ────────────────────────────────────────────────
    //

    pub async fn multi_get(
        &self,
        stream: &Stream,
        topics: &[Topic],
        accept: Accept,
        token: &str,
    ) -> Result<Vec<MultiGetItem>, HttpError> {
        ensure_non_empty("token", token)?;

        let inner_ct = match accept {
            Accept::TextPlain => "text/plain",
            Accept::Base64 => "base64",
            _ => {
                return Err(HttpError::InvalidInput(
                    "multi-get only supports Accept::TextPlain or Accept::Base64".into(),
                ))
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

        let mut url = self.base_url.clone();
        url.set_path("data/v0/multi");

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
                body: resp.text().await.unwrap_or_default(),
            });
        }

        let text = resp.text().await?;
        let items: Vec<MultiGetItem> = serde_json::from_str(&text)?;

        Ok(items)
    }
}

//
// ────────────────────────────────────────────────────────────────────────────────
//   INTERNAL HELPERS
// ────────────────────────────────────────────────────────────────────────────────
//

fn ensure_non_empty(name: &str, s: &str) -> Result<(), HttpError> {
    if s.trim().is_empty() {
        return Err(HttpError::InvalidInput(format!("{name} must not be empty")));
    }
    Ok(())
}