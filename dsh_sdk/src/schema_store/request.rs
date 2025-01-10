use log::trace;

use crate::Dsh;

use super::SchemaStoreError;

const DEFAULT_CONTENT_TYPE: &str = "application/vnd.schemaregistry.v1+json";

pub trait Request {
    fn new_client() -> Self;
    fn get_request<R>(
        &self,
        url: String,
    ) -> impl std::future::Future<Output = Result<R, SchemaStoreError>> + Send
    where
        R: serde::de::DeserializeOwned;
    fn get_request_plain(
        &self,
        url: String,
    ) -> impl std::future::Future<Output = Result<String, SchemaStoreError>> + Send;
    fn post_request<R, B>(
        &self,
        url: String,
        body: B,
    ) -> impl std::future::Future<Output = Result<R, SchemaStoreError>> + Send
    where
        R: serde::de::DeserializeOwned,
        B: serde::Serialize + Send;
    fn put_request<R, B>(
        &self,
        url: String,
        body: B,
    ) -> impl std::future::Future<Output = Result<R, SchemaStoreError>> + Send
    where
        R: serde::de::DeserializeOwned,
        B: serde::Serialize + Send;
}

impl Request for reqwest::Client {
    fn new_client() -> Self {
        // TODO: replace with hyper client
        Dsh::get()
            .reqwest_client_config()
            .build()
            .expect("Failed to build reqwest client")
    }
    async fn get_request<R>(&self, url: String) -> Result<R, SchemaStoreError>
    where
        R: serde::de::DeserializeOwned,
    {
        trace!("GET {}", url);
        let request = self
            .get(&url)
            .header("Content-Type", DEFAULT_CONTENT_TYPE)
            .header("Accept", DEFAULT_CONTENT_TYPE);
        let response = request.send().await?;
        trace!("Response: {:?}", response);
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(SchemaStoreError::InvalidStatusCode {
                status_code: response.status().as_u16(),
                url: url.to_string(),
                error: response.text().await.unwrap_or_default(),
            })
        }
    }

    async fn get_request_plain(&self, url: String) -> Result<String, SchemaStoreError> {
        trace!("GET {}", url);
        let request = self.get(&url);
        let response = request.send().await?;
        trace!("Response: {:?}", response);
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(SchemaStoreError::InvalidStatusCode {
                status_code: response.status().as_u16(),
                url: url.to_string(),
                error: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Helper function to send a POST request and return the response with the expected type (serde with as JSON)
    async fn post_request<R, B>(&self, url: String, body: B) -> Result<R, SchemaStoreError>
    where
        R: serde::de::DeserializeOwned,
        B: serde::Serialize + Send,
    {
        trace!("POST {}", url);
        let json_body = serde_json::to_vec(&body)?;
        let request = self
            .post(&url)
            .body(json_body)
            .header("Content-Type", DEFAULT_CONTENT_TYPE)
            .header("Accept", DEFAULT_CONTENT_TYPE);
        let response = request.send().await?;
        trace!("Response {:?}", response);
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(SchemaStoreError::InvalidStatusCode {
                status_code: response.status().as_u16(),
                url: url.to_string(),
                error: response.text().await.unwrap_or_default(),
            })
        }
    }

    /// Helper function to send a PUT request and return the response with the expected type (serde with as JSON)
    async fn put_request<R, B>(&self, url: String, body: B) -> Result<R, SchemaStoreError>
    where
        R: serde::de::DeserializeOwned,
        B: serde::Serialize + Send,
    {
        trace!("PUT {}", url);
        let json_body = serde_json::to_vec(&body)?;
        let request = self
            .put(&url)
            .body(json_body)
            .header("Content-Type", DEFAULT_CONTENT_TYPE)
            .header("Accept", DEFAULT_CONTENT_TYPE);
        let response = request.send().await?;
        trace!("Response {:?}", response);
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(SchemaStoreError::InvalidStatusCode {
                status_code: response.status().as_u16(),
                url: url.to_string(),
                error: response.text().await.unwrap_or_default(),
            })
        }
    }
}
