//! Module for fetching and storing access tokens for the DSH Rest API client
//!
//! This module is meant to be used together with the [dsh_rest_api_client].
//!
//! The TokenFetcher will fetch and store access tokens to be used in the DSH Rest API client.
//!
//! ## Example
//! Recommended usage is to use the [RestTokenFetcherBuilder] to create a new instance of the token fetcher.
//! However, you can also create a new instance of the token fetcher directly.
//! ```no_run
//! use dsh_sdk::{RestTokenFetcherBuilder, Platform};
//! use dsh_rest_api_client::Client;
//!
//! const CLIENT_SECRET: &str = "";
//! const TENANT: &str = "tenant-name";
//!
//! #[tokio::main]
//! async fn main() {
//!     let platform = Platform::NpLz;
//!     let client = Client::new(platform.endpoint_rest_api());
//!
//!     let tf = RestTokenFetcherBuilder::new(platform)
//!         .tenant_name(TENANT.to_string())
//!         .client_secret(CLIENT_SECRET.to_string())
//!         .build()
//!         .unwrap();
//!
//!     let response = client
//!         .topic_get_by_tenant_topic(TENANT, &tf.get_token().await.unwrap())
//!         .await;
//!     println!("Available topics: {:#?}", response);
//! }
//! ```

use std::fmt::Debug;
use std::ops::Add;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use log::debug;
use serde::Deserialize;

use crate::error::DshRestTokenError;
use crate::utils::Platform;

/// Access token of the authentication serveice of DSH.
///
/// This is the response whem requesting for a new access token.
///
/// ## Recommended usage
/// Use the [RestTokenFetcher::get_token] to get the bearer token, the `TokenFetcher` will automatically fetch a new token if the current token is not valid.
#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    access_token: String,
    expires_in: u64,
    refresh_expires_in: u32,
    token_type: String,
    #[serde(rename(deserialize = "not-before-policy"))]
    not_before_policy: u32,
    scope: String,
}

impl AccessToken {
    /// Get the formatted token
    pub fn formatted_token(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }

    /// Get the access token
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Get the expires in of the access token
    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }

    /// Get the refresh expires in of the access token
    pub fn refresh_expires_in(&self) -> u32 {
        self.refresh_expires_in
    }

    /// Get the token type of the access token
    pub fn token_type(&self) -> &str {
        &self.token_type
    }

    /// Get the not before policy of the access token
    pub fn not_before_policy(&self) -> u32 {
        self.not_before_policy
    }

    /// Get the scope of the access token
    pub fn scope(&self) -> &str {
        &self.scope
    }
}

impl Default for AccessToken {
    fn default() -> Self {
        Self {
            access_token: "".to_string(),
            expires_in: 0,
            refresh_expires_in: 0,
            token_type: "".to_string(),
            not_before_policy: 0,
            scope: "".to_string(),
        }
    }
}

/// Fetch and store access tokens to be used in the DSH Rest API client
///
/// This struct will fetch and store access tokens to be used in the DSH Rest API client.
/// It will automatically fetch a new token if the current token is not valid.
pub struct RestTokenFetcher {
    access_token: Mutex<AccessToken>,
    fetched_at: Mutex<Instant>,
    client_id: String,
    client_secret: String,
    client: reqwest::Client,
    auth_url: String,
}

impl RestTokenFetcher {
    /// Create a new instance of the token fetcher
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::{RestTokenFetcher, Platform};
    /// use dsh_rest_api_client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let platform = Platform::NpLz;
    ///     let client_id = platform.rest_client_id("my-tenant");
    ///     let client_secret = "my-secret".to_string();
    ///     let token_fetcher = RestTokenFetcher::new(client_id, client_secret, platform.endpoint_rest_access_token().to_string());
    ///     let token = token_fetcher.get_token().await.unwrap();
    /// }
    /// ```
    pub fn new(client_id: String, client_secret: String, auth_url: String) -> Self {
        Self::new_with_client(
            client_id,
            client_secret,
            auth_url,
            reqwest::Client::default(),
        )
    }

    /// Create a new instance of the token fetcher with custom reqwest client
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::{RestTokenFetcher, Platform};
    /// use dsh_rest_api_client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let platform = Platform::NpLz;
    ///     let client_id = platform.rest_client_id("my-tenant");
    ///     let client_secret = "my-secret".to_string();
    ///     let client = reqwest::Client::new();
    ///     let token_fetcher = RestTokenFetcher::new_with_client(client_id, client_secret, platform.endpoint_rest_access_token().to_string(), client);
    ///     let token = token_fetcher.get_token().await.unwrap();
    /// }
    /// ```
    pub fn new_with_client(
        client_id: String,
        client_secret: String,
        auth_url: String,
        client: reqwest::Client,
    ) -> Self {
        Self {
            access_token: Mutex::new(AccessToken::default()),
            fetched_at: Mutex::new(Instant::now()),
            client_id,
            client_secret,
            client,
            auth_url,
        }
    }

    /// Get token from the token fetcher
    ///
    /// If the cached token is not valid, it will fetch a new token from the server.
    /// It will return the token as a string, formatted as "{token_type} {token}"
    /// If the request fails for a new token, it will return a [DshRestTokenError::FailureTokenFetch] error.
    /// This will contain the underlying reqwest error.
    pub async fn get_token(&self) -> Result<String, DshRestTokenError> {
        match self.is_valid() {
            true => Ok(self.access_token.lock().unwrap().formatted_token()),
            false => {
                debug!("Token is expired, fetching new token");
                let access_token = self.fetch_access_token_from_server().await?;
                let mut token = self.access_token.lock().unwrap();
                let mut fetched_at = self.fetched_at.lock().unwrap();
                *token = access_token;
                *fetched_at = Instant::now();
                Ok(token.formatted_token())
            }
        }
    }

    /// Check if the current access token is still valid
    ///
    /// If the token has expired, it will return false.
    pub fn is_valid(&self) -> bool {
        let access_token = self.access_token.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = AccessToken::default();
            self.access_token.clear_poison();
            e.into_inner()
        });
        let fetched_at = self.fetched_at.lock().unwrap_or_else(|e| {
            self.fetched_at.clear_poison();
            e.into_inner()
        });
        // Check if expires in has elapsed (+ safety margin of 5 seconds)
        fetched_at.elapsed().add(Duration::from_secs(5))
            < Duration::from_secs(access_token.expires_in)
    }

    /// Fetch a new access token from the server
    ///
    /// This will fetch a new access token from the server and return it.
    /// If the request fails, it will return a [DshRestTokenError::FailureTokenFetch] error.
    /// If the status code is not successful, it will return a [DshRestTokenError::StatusCode] error.
    /// If the request is successful, it will return the [AccessToken].
    pub async fn fetch_access_token_from_server(&self) -> Result<AccessToken, DshRestTokenError> {
        let response = self
            .client
            .post(&self.auth_url)
            .form(&[
                ("client_id", self.client_id.as_ref()),
                ("client_secret", self.client_secret.as_ref()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await
            .map_err(DshRestTokenError::FailureTokenFetch)?;
        if !response.status().is_success() {
            Err(DshRestTokenError::StatusCode {
                status_code: response.status(),
                error_body: response,
            })
        } else {
            response
                .json::<AccessToken>()
                .await
                .map_err(DshRestTokenError::FailureTokenFetch)
        }
    }
}

impl Debug for RestTokenFetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RestTokenFetcher")
            .field("access_token", &self.access_token)
            .field("fetched_at", &self.fetched_at)
            .field("client_id", &self.client_id)
            .field("client_secret", &"xxxxxx")
            .field("auth_url", &self.auth_url)
            .finish()
    }
}

/// Builder for the token fetcher
pub struct RestTokenFetcherBuilder {
    client: Option<reqwest::Client>,
    client_id: Option<String>,
    client_secret: Option<String>,
    platform: Platform,
    tenant_name: Option<String>,
}

impl RestTokenFetcherBuilder {
    /// Get a new instance of the ClientBuilder
    ///
    /// # Arguments
    /// * `platform` - The target platform to use for the token fetcher
    pub fn new(platform: Platform) -> Self {
        Self {
            client: None,
            client_id: None,
            client_secret: None,
            platform,
            tenant_name: None,
        }
    }

    /// Set the client_id for the client
    ///
    /// Alternatively, set `tenant_name` to generate the client_id.
    /// `Client_id` does have precedence over `tenant_name`.
    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = Some(client_id);
        self
    }

    /// Set the client_secret for the client
    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = Some(client_secret);
        self
    }

    /// Set the tenant_name for the client, this will generate the client_id
    ///
    /// Alternatively, set `client_id` directly.
    /// `Tenant_name` does have precedence over `client_id`.
    pub fn tenant_name(mut self, tenant_name: String) -> Self {
        self.tenant_name = Some(tenant_name);
        self
    }

    /// Provide a custom configured Reqwest client for the token
    ///
    /// This is optional, if not provided, a default client will be used.
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Build the client and token fetcher
    ///
    /// This will build the client and token fetcher based on the given parameters.
    /// It will return a tuple with the client and token fetcher.
    ///
    /// ## Example
    /// ```
    /// # use dsh_sdk::{RestTokenFetcherBuilder, Platform};
    /// let platform = Platform::NpLz;
    /// let client_id = "robot:dev-lz-dsh:my-tenant".to_string();
    /// let client_secret = "secret".to_string();
    /// let tf = RestTokenFetcherBuilder::new(platform)
    ///     .client_id(client_id)
    ///     .client_secret(client_secret)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<RestTokenFetcher, DshRestTokenError> {
        let client_secret = self
            .client_secret
            .ok_or(DshRestTokenError::UnknownClientSecret)?;
        let client_id = self
            .client_id
            .or_else(|| {
                self.tenant_name
                    .as_ref()
                    .map(|tenant_name| self.platform.rest_client_id(tenant_name))
            })
            .ok_or(DshRestTokenError::UnknownClientId)?;
        let client = self.client.unwrap_or_default();
        let token_fetcher = RestTokenFetcher::new_with_client(
            client_id,
            client_secret,
            self.platform.endpoint_rest_access_token().to_string(),
            client,
        );
        Ok(token_fetcher)
    }
}

impl Debug for RestTokenFetcherBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let client_secret = self
            .client_secret
            .as_ref()
            .map(|_| "Some(\"client_secret\")");
        f.debug_struct("RestTokenFetcherBuilder")
            .field("client_id", &self.client_id)
            .field("client_secret", &client_secret)
            .field("platform", &self.platform)
            .field("tenant_name", &self.tenant_name)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_mock_tf() -> RestTokenFetcher {
        RestTokenFetcher {
            access_token: Mutex::new(AccessToken::default()),
            fetched_at: Mutex::new(Instant::now()),
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            client: reqwest::Client::new(),
            auth_url: "http://localhost".to_string(),
        }
    }

    #[test]
    fn test_access_token() {
        let token_str = r#"{
          "access_token": "secret_access_token",
          "expires_in": 600,
          "refresh_expires_in": 0,
          "token_type": "Bearer",
          "not-before-policy": 0,
          "scope": "email"
        }"#;
        let token: AccessToken = serde_json::from_str(token_str).unwrap();
        assert_eq!(token.access_token(), "secret_access_token");
        assert_eq!(token.expires_in(), 600);
        assert_eq!(token.refresh_expires_in(), 0);
        assert_eq!(token.token_type(), "Bearer");
        assert_eq!(token.not_before_policy(), 0);
        assert_eq!(token.scope(), "email");
        assert_eq!(token.formatted_token(), "Bearer secret_access_token");
    }

    #[test]
    fn test_access_token_default() {
        let token = AccessToken::default();
        assert_eq!(token.access_token(), "");
        assert_eq!(token.expires_in(), 0);
        assert_eq!(token.refresh_expires_in(), 0);
        assert_eq!(token.token_type(), "");
        assert_eq!(token.not_before_policy(), 0);
        assert_eq!(token.scope(), "");
        assert_eq!(token.formatted_token(), " ");
    }

    #[test]
    fn test_rest_token_fetcher_is_valid_default_token() {
        // Test is_valid when validating default token (should expire in 0 seconds)
        let tf = create_mock_tf();
        assert!(!tf.is_valid());
    }

    #[test]
    fn test_rest_token_fetcher_is_valid_valid_token() {
        let tf = create_mock_tf();
        tf.access_token.lock().unwrap().expires_in = 600;
        assert!(tf.is_valid());
    }

    #[test]
    fn test_rest_token_fetcher_is_valid_expired_token() {
        // Test is_valid when validating an expired token
        let tf = create_mock_tf();
        tf.access_token.lock().unwrap().expires_in = 600;
        *tf.fetched_at.lock().unwrap() = Instant::now() - Duration::from_secs(600);
        assert!(!tf.is_valid());
    }

    #[test]
    fn test_rest_token_fetcher_is_valid_poisoned_token() {
        // Test is_valid when token is poisoned
        let tf = create_mock_tf();
        tf.access_token.lock().unwrap().expires_in = 600;
        let tf_arc = std::sync::Arc::new(tf);
        let tf_clone = tf_arc.clone();
        assert!(tf_arc.is_valid(), "Token should be valid");
        let h = std::thread::spawn(move || {
            let _unused = tf_clone.access_token.lock().unwrap();
            panic!("Poison token")
        });
        let _ = h.join();
        assert!(!tf_arc.is_valid(), "Token should be invalid");
    }

    #[tokio::test]
    async fn test_fetch_access_token_from_server() {
        let mut auth_server = mockito::Server::new_async().await;
        auth_server
            .mock("POST", "/")
            .with_status(200)
            .with_body(
                r#"{
          "access_token": "secret_access_token",
          "expires_in": 600,
          "refresh_expires_in": 0,
          "token_type": "Bearer",
          "not-before-policy": 0,
          "scope": "email"
        }"#,
            )
            .create();
        let mut tf = create_mock_tf();
        tf.auth_url = auth_server.url();
        let token = tf.fetch_access_token_from_server().await.unwrap();
        assert_eq!(token.access_token(), "secret_access_token");
        assert_eq!(token.expires_in(), 600);
        assert_eq!(token.refresh_expires_in(), 0);
        assert_eq!(token.token_type(), "Bearer");
        assert_eq!(token.not_before_policy(), 0);
        assert_eq!(token.scope(), "email");
    }

    #[tokio::test]
    async fn test_fetch_access_token_from_server_error() {
        let mut auth_server = mockito::Server::new_async().await;
        auth_server
            .mock("POST", "/")
            .with_status(400)
            .with_body("Bad request")
            .create();
        let mut tf = create_mock_tf();
        tf.auth_url = auth_server.url();
        let err = tf.fetch_access_token_from_server().await.unwrap_err();
        match err {
            DshRestTokenError::StatusCode {
                status_code,
                error_body,
            } => {
                assert_eq!(status_code, reqwest::StatusCode::BAD_REQUEST);
                assert_eq!(error_body.text().await.unwrap(), "Bad request");
            }
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    #[test]
    fn test_token_fetcher_builder_client_id() {
        let platform = Platform::NpLz;
        let client_id = "robot:dev-lz-dsh:my-tenant";
        let client_secret = "secret";
        let tf = RestTokenFetcherBuilder::new(platform)
            .client_id(client_id.to_string())
            .client_secret(client_secret.to_string())
            .build()
            .unwrap();
        assert_eq!(tf.client_id, client_id);
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_rest_access_token());
    }

    #[test]
    fn test_token_fetcher_builder_tenant_name() {
        let platform = Platform::NpLz;
        let tenant_name = "my-tenant";
        let client_secret = "secret";
        let tf = RestTokenFetcherBuilder::new(platform)
            .tenant_name(tenant_name.to_string())
            .client_secret(client_secret.to_string())
            .build()
            .unwrap();
        assert_eq!(
            tf.client_id,
            format!("robot:{}:{}", Platform::NpLz.realm(), tenant_name)
        );
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_rest_access_token());
    }

    #[test]
    fn test_token_fetcher_builder_custom_client() {
        let platform = Platform::NpLz;
        let client_id = "robot:dev-lz-dsh:my-tenant";
        let client_secret = "secret";
        let custom_client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
        let tf = RestTokenFetcherBuilder::new(platform)
            .client_id(client_id.to_string())
            .client_secret(client_secret.to_string())
            .client(custom_client.clone())
            .build()
            .unwrap();
        assert_eq!(tf.client_id, client_id);
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_rest_access_token());
    }

    #[test]
    fn test_token_fetcher_builder_client_id_precedence() {
        let platform = Platform::NpLz;
        let tenant = "my-tenant";
        let client_id_override = "override";
        let client_secret = "secret";
        let tf = RestTokenFetcherBuilder::new(platform)
            .tenant_name(tenant.to_string())
            .client_id(client_id_override.to_string())
            .client_secret(client_secret.to_string())
            .build()
            .unwrap();
        assert_eq!(tf.client_id, client_id_override);
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_rest_access_token());
    }

    #[test]
    fn test_token_fetcher_builder_build_error() {
        let err = RestTokenFetcherBuilder::new(Platform::NpLz)
            .client_secret("client_secret".to_string())
            .build()
            .unwrap_err();
        assert!(matches!(err, DshRestTokenError::UnknownClientId));

        let err = RestTokenFetcherBuilder::new(Platform::NpLz)
            .tenant_name("tenant_name".to_string())
            .build()
            .unwrap_err();
        assert!(matches!(err, DshRestTokenError::UnknownClientSecret));
    }
}
