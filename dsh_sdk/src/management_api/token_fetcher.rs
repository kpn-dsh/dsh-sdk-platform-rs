//! Management API token fetching for DSH.
//!
//! This module provides an interface (`ManagementApiTokenFetcher`) for fetching and
//! caching access tokens required to communicate with DSH’s management (REST) API.
//! Access tokens are automatically refreshed when expired, allowing seamless
//! integrations with the DSH platform.
//!
//! # Overview
//! - **[`AccessToken`]**: Access token from the authentication server.  
//! - **[`ManagementApiTokenFetcher`]**: A token fetcher that caches tokens and
//!   refreshes them upon expiration.  
//! - **[`ManagementApiTokenFetcherBuilder`]**: A builder for customizing the fetcher’s
//!   client, credentials, and target platform.  
//!
//! # Typical Usage
//! 1. **Instantiate** a fetcher with credentials:  
//!    ```
//!    use dsh_sdk::management_api::ManagementApiTokenFetcherBuilder;
//!    use dsh_sdk::Platform;
//!
//!    let platform = Platform::NpLz;
//!    let token_fetcher = ManagementApiTokenFetcherBuilder::new(platform)
//!         .tenant_name("my-tenant")
//!         .client_secret("my-secret")
//!         .build()
//!         .unwrap();
//!    ```
//! 2. **Fetch** the token when needed:  
//!    ```ignore
//!    let token = token_fetcher.get_token().await?;
//!    ```
//! 3. **Reuse** the same fetcher for subsequent calls; it auto-refreshes tokens.  
//!
//! For more advanced usage (custom [`reqwest::Client`] or different credential sourcing),
//! see [`ManagementApiTokenFetcher::new_with_client`] or the [`ManagementApiTokenFetcherBuilder`].

use std::fmt::Debug;
use std::ops::Add;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use log::debug;
use serde::Deserialize;

use super::error::ManagementApiTokenError;
use crate::utils::Platform;

/// Represents the Access Token by DSH’s authentication service.
///
/// The fields include information about the token’s validity window,
/// token type, and scope. Typically, you won’t instantiate `AccessToken` directly:
/// use [`ManagementApiTokenFetcher::get_token`](crate::management_api::ManagementApiTokenFetcher::get_token)
/// to automatically obtain or refresh a valid token.
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
    /// Returns a complete token string, i.e. `"{token_type} {access_token}"`.
    pub fn formatted_token(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }

    /// Returns the raw access token string (without the token type).
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Returns the number of seconds until this token expires.
    pub fn expires_in(&self) -> u64 {
        self.expires_in
    }

    /// Returns the number of seconds until the refresh token expires.
    pub fn refresh_expires_in(&self) -> u32 {
        self.refresh_expires_in
    }

    /// Returns the token type (e.g., `"Bearer"`).
    pub fn token_type(&self) -> &str {
        &self.token_type
    }

    /// Returns the “not before” policy timestamp from the authentication server.
    pub fn not_before_policy(&self) -> u32 {
        self.not_before_policy
    }

    /// Returns the scope string (e.g., `"email"`).
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

/// A fetcher for obtaining and storing access tokens, enabling authenticated
/// requests to DSH’s management (REST) API.
///
/// This struct caches the token in memory and refreshes it automatically
/// once expired.
///
/// # Usage
/// - **`new`**: Construct a fetcher with provided credentials.  
/// - **`new_with_client`**: Provide a custom [`reqwest::Client`] if needed.  
/// - **`get_token`**: Returns the current token if still valid, or fetches a new one.  
///
/// # Example
/// ```no_run
/// use dsh_sdk::management_api::ManagementApiTokenFetcher;
/// use dsh_sdk::Platform;
///
/// # use std::error::Error;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn Error>> {
/// let platform = Platform::NpLz;
/// let client_id = platform.management_api_client_id("my-tenant");
/// let client_secret = "my-secret".to_string();
/// let token_fetcher = ManagementApiTokenFetcher::new(
///     client_id,
///     client_secret,
///     platform.endpoint_management_api_token().to_string()
/// );
///
/// let token = token_fetcher.get_token().await?;
/// println!("Obtained token: {}", token);
/// # Ok(())
/// # }
/// ```
pub struct ManagementApiTokenFetcher {
    access_token: Mutex<AccessToken>,
    fetched_at: Mutex<Instant>,
    client_id: String,
    client_secret: String,
    client: reqwest::Client,
    auth_url: String,
}

impl ManagementApiTokenFetcher {
    /// Creates a new token fetcher.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::management_api::ManagementApiTokenFetcher;
    /// use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let platform = Platform::NpLz;
    /// let client_id = platform.management_api_client_id("my-tenant");
    /// let client_secret = "my-secret";
    /// let token_fetcher = ManagementApiTokenFetcher::new(
    ///     client_id,
    ///     client_secret,
    ///     platform.endpoint_management_api_token()
    /// );
    ///
    /// let token = token_fetcher.get_token().await.unwrap();
    /// println!("Token: {}", token);
    /// # }
    /// ```
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        auth_url: impl Into<String>,
    ) -> Self {
        Self::new_with_client(
            client_id,
            client_secret,
            auth_url,
            reqwest::Client::default(),
        )
    }

    /// Returns a [`ManagementApiTokenFetcherBuilder`] for more flexible creation
    /// of a token fetcher (e.g., specifying a custom client).
    pub fn builder(platform: Platform) -> ManagementApiTokenFetcherBuilder {
        ManagementApiTokenFetcherBuilder::new(platform)
    }

    /// Creates a new fetcher with a **custom** [`reqwest::Client`].
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::management_api::ManagementApiTokenFetcher;
    /// use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let platform = Platform::NpLz;
    /// let client_id = platform.management_api_client_id("my-tenant");
    /// let client_secret = "my-secret";
    /// let custom_client = reqwest::Client::new();
    /// let token_fetcher = ManagementApiTokenFetcher::new_with_client(
    ///     client_id,
    ///     client_secret,
    ///     platform.endpoint_management_api_token().to_string(),
    ///     custom_client
    /// );
    /// let token = token_fetcher.get_token().await.unwrap();
    /// println!("Token: {}", token);
    /// # }
    /// ```
    pub fn new_with_client(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        auth_url: impl Into<String>,
        client: reqwest::Client,
    ) -> Self {
        Self {
            access_token: Mutex::new(AccessToken::default()),
            fetched_at: Mutex::new(Instant::now()),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            client,
            auth_url: auth_url.into(),
        }
    }

    /// Obtains the token from cache if still valid, otherwise fetches a new one.
    ///
    /// The returned string is formatted as `"{token_type} {access_token}"`.
    ///
    /// # Errors
    /// - [`ManagementApiTokenError::FailureTokenFetch`]:
    ///   If the network request fails or times out when fetching a new token.
    /// - [`ManagementApiTokenError::StatusCode`]:
    ///   If the authentication server returns a non-success HTTP status code.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::management_api::ManagementApiTokenFetcher;
    /// # #[tokio::main]
    /// # async fn main() {
    ///     let tf = ManagementApiTokenFetcher::new(
    ///         "client_id".to_string(),
    ///         "client_secret".to_string(),
    ///         "http://example.com/auth".to_string()
    ///     );
    ///     match tf.get_token().await {
    ///         Ok(token) => println!("Got token: {}", token),
    ///         Err(e) => eprintln!("Error fetching token: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_token(&self) -> Result<String, ManagementApiTokenError> {
        if self.is_valid() {
            Ok(self.access_token.lock().unwrap().formatted_token())
        } else {
            debug!("Token is expired, fetching new token");
            let access_token = self.fetch_access_token_from_server().await?;
            let mut token = self.access_token.lock().unwrap();
            let mut fetched_at = self.fetched_at.lock().unwrap();
            *token = access_token;
            *fetched_at = Instant::now();
            Ok(token.formatted_token())
        }
    }

    /// Determines if the internally cached token is still valid.
    ///
    /// A token is considered valid if its remaining lifetime
    /// (minus a 5-second safety margin) is greater than zero.
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
        // Check if 'expires_in' has elapsed (+ 5-second safety margin)
        fetched_at.elapsed().add(Duration::from_secs(5))
            < Duration::from_secs(access_token.expires_in)
    }

    /// Fetches a fresh `AccessToken` from the authentication server.
    ///
    /// # Errors
    /// - [`ManagementApiTokenError::FailureTokenFetch`]:
    ///   If the network request fails or times out.
    /// - [`ManagementApiTokenError::StatusCode`]:
    ///   If the server returns a non-success status code.
    pub async fn fetch_access_token_from_server(
        &self,
    ) -> Result<AccessToken, ManagementApiTokenError> {
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
            .map_err(ManagementApiTokenError::FailureTokenFetch)?;

        if !response.status().is_success() {
            Err(ManagementApiTokenError::StatusCode {
                status_code: response.status(),
                error_body: response.text().await.unwrap_or_default(),
            })
        } else {
            response
                .json::<AccessToken>()
                .await
                .map_err(ManagementApiTokenError::FailureTokenFetch)
        }
    }
}

impl Debug for ManagementApiTokenFetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagementApiTokenFetcher")
            .field("access_token", &self.access_token)
            .field("fetched_at", &self.fetched_at)
            .field("client_id", &self.client_id)
            // For security, obfuscate the secret
            .field("client_secret", &"xxxxxx")
            .field("auth_url", &self.auth_url)
            .finish()
    }
}

/// A builder for constructing a [`ManagementApiTokenFetcher`].
///
/// This builder allows customization of the token fetcher by specifying:
/// - **client_id** or **tenant_name** (tenant name is used to generate the client_id)
/// - **client_secret**
/// - **custom [`reqwest::Client`]** (optional)
/// - **platform** (e.g., [`Platform::NpLz`] or [`Platform::Poc`])
///
/// # Example
/// ```
/// use dsh_sdk::management_api::ManagementApiTokenFetcherBuilder;
/// use dsh_sdk::Platform;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let platform = Platform::NpLz;
/// let client_id = "robot:dev-lz-dsh:my-tenant".to_string();
/// let client_secret = "secret".to_string();
/// let token_fetcher = ManagementApiTokenFetcherBuilder::new(platform)
///     .client_id(client_id)
///     .client_secret(client_secret)
///     .build()?;
/// // Use `token_fetcher`
/// # Ok(())
/// # }
/// ```
pub struct ManagementApiTokenFetcherBuilder {
    client: Option<reqwest::Client>,
    client_id: Option<String>,
    client_secret: Option<String>,
    platform: Platform,
    tenant_name: Option<String>,
}

impl ManagementApiTokenFetcherBuilder {
    /// Creates a new builder configured for the specified [`Platform`].
    ///
    /// # Arguments
    /// - `platform`: The target platform (e.g., `Platform::NpLz`) to determine
    ///   default endpoints for fetching tokens.
    pub fn new(platform: Platform) -> Self {
        Self {
            client: None,
            client_id: None,
            client_secret: None,
            platform,
            tenant_name: None,
        }
    }

    /// Sets an explicit client ID for authentication.
    ///
    /// If you also specify `tenant_name`, the client ID here takes precedence.
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Sets a client secret required for token fetching.
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    /// Sets a tenant name from which the client ID will be derived.
    ///
    /// This will use `platform.rest_client_id(tenant_name)` unless `client_id`
    /// is already set.
    pub fn tenant_name(mut self, tenant_name: impl Into<String>) -> Self {
        self.tenant_name = Some(tenant_name.into());
        self
    }

    /// Supplies a custom [`reqwest::Client`] if you need specialized settings
    /// (e.g., proxy configuration, timeouts, etc.).
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Builds a [`ManagementApiTokenFetcher`] based on the provided configuration.
    ///
    /// # Errors
    /// - [`ManagementApiTokenError::UnknownClientSecret`]:
    ///   If the client secret is unset.
    /// - [`ManagementApiTokenError::UnknownClientId`]:
    ///   If neither `client_id` nor `tenant_name` is provided.
    ///
    /// # Example
    /// ```
    /// use dsh_sdk::management_api::{ManagementApiTokenFetcherBuilder, ManagementApiTokenError};
    /// use dsh_sdk::Platform;
    ///
    /// # fn main() -> Result<(), ManagementApiTokenError> {
    /// let fetcher = ManagementApiTokenFetcherBuilder::new(Platform::NpLz)
    ///     .client_id("robot:dev-lz-dsh:my-tenant".to_string())
    ///     .client_secret("secret".to_string())
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<ManagementApiTokenFetcher, ManagementApiTokenError> {
        let client_secret = self
            .client_secret
            .ok_or(ManagementApiTokenError::UnknownClientSecret)?;

        let client_id = self
            .client_id
            .or_else(|| {
                self.tenant_name
                    .as_ref()
                    .map(|tenant_name| self.platform.management_api_client_id(tenant_name))
            })
            .ok_or(ManagementApiTokenError::UnknownClientId)?;

        let client = self.client.unwrap_or_default();
        let token_fetcher = ManagementApiTokenFetcher::new_with_client(
            client_id,
            client_secret,
            self.platform.endpoint_management_api_token().to_string(),
            client,
        );
        Ok(token_fetcher)
    }
}

impl Debug for ManagementApiTokenFetcherBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let client_secret = self
            .client_secret
            .as_ref()
            .map(|_| "Some(\"client_secret\")");
        f.debug_struct("ManagementApiTokenFetcherBuilder")
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

    fn create_mock_tf() -> ManagementApiTokenFetcher {
        ManagementApiTokenFetcher {
            access_token: Mutex::new(AccessToken::default()),
            fetched_at: Mutex::new(Instant::now()),
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            client: reqwest::Client::new(),
            auth_url: "http://localhost".to_string(),
        }
    }

    /// Ensures `AccessToken` is properly deserialized and returns expected fields.
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

    /// Validates the default constructor yields an empty `AccessToken`.
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

    /// Verifies that a default token is considered invalid since it expires immediately.
    #[test]
    fn test_rest_token_fetcher_is_valid_default_token() {
        let tf = create_mock_tf();
        assert!(!tf.is_valid(), "Default token should be invalid");
    }

    /// Demonstrates that `is_valid` returns true if a token is configured with future expiration.
    #[test]
    fn test_rest_token_fetcher_is_valid_valid_token() {
        let tf = create_mock_tf();
        tf.access_token.lock().unwrap().expires_in = 600;
        assert!(
            tf.is_valid(),
            "Token with 600s lifetime should be valid initially"
        );
    }

    /// Confirms `is_valid` returns false after the token’s entire lifetime has elapsed.
    #[test]
    fn test_rest_token_fetcher_is_valid_expired_token() {
        let tf = create_mock_tf();
        tf.access_token.lock().unwrap().expires_in = 600;
        *tf.fetched_at.lock().unwrap() = Instant::now() - Duration::from_secs(600);
        assert!(!tf.is_valid(), "Token should expire after 600s have passed");
    }

    /// Tests behavior when a token is “poisoned” (i.e., panicked while locked).
    #[test]
    fn test_rest_token_fetcher_is_valid_poisoned_token() {
        let tf = create_mock_tf();
        tf.access_token.lock().unwrap().expires_in = 600;
        let tf_arc = std::sync::Arc::new(tf);
        let tf_clone = tf_arc.clone();
        assert!(tf_arc.is_valid(), "Token should be valid before poison");
        let handle = std::thread::spawn(move || {
            let _unused = tf_clone.access_token.lock().unwrap();
            panic!("Poison token");
        });
        let _ = handle.join();
        assert!(
            !tf_arc.is_valid(),
            "Token should be reset to default after poisoning"
        );
    }

    /// Checks success scenario for fetching a new token from a mock server.
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
    }

    /// Checks that an HTTP 400 response is handled as an error.
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
            ManagementApiTokenError::StatusCode {
                status_code,
                error_body,
            } => {
                assert_eq!(status_code, reqwest::StatusCode::BAD_REQUEST);
                assert_eq!(error_body, "Bad request");
            }
            _ => panic!("Unexpected error: {:?}", err),
        }
    }

    /// Ensures the builder sets `client_id` explicitly.
    #[test]
    fn test_token_fetcher_builder_client_id() {
        let platform = Platform::NpLz;
        let client_id = "robot:dev-lz-dsh:my-tenant";
        let client_secret = "secret";
        let tf = ManagementApiTokenFetcherBuilder::new(platform)
            .client_id(client_id.to_string())
            .client_secret(client_secret.to_string())
            .build()
            .unwrap();
        assert_eq!(tf.client_id, client_id);
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_management_api_token());
    }

    /// Ensures the builder can auto-generate `client_id` from the `tenant_name`.
    #[test]
    fn test_token_fetcher_builder_tenant_name() {
        let platform = Platform::NpLz;
        let tenant_name = "my-tenant";
        let client_secret = "secret";
        let tf = ManagementApiTokenFetcherBuilder::new(platform)
            .tenant_name(tenant_name.to_string())
            .client_secret(client_secret.to_string())
            .build()
            .unwrap();
        assert_eq!(
            tf.client_id,
            format!("robot:{}:{}", Platform::NpLz.realm(), tenant_name)
        );
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_management_api_token());
    }

    /// Validates that a custom `reqwest::Client` can be injected into the builder.
    #[test]
    fn test_token_fetcher_builder_custom_client() {
        let platform = Platform::NpLz;
        let client_id = "robot:dev-lz-dsh:my-tenant";
        let client_secret = "secret";
        let custom_client = reqwest::Client::builder()
            .tls_backend_rustls()
            .build()
            .unwrap();
        let tf = ManagementApiTokenFetcherBuilder::new(platform)
            .client_id(client_id.to_string())
            .client_secret(client_secret.to_string())
            .client(custom_client.clone())
            .build()
            .unwrap();
        assert_eq!(tf.client_id, client_id);
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_management_api_token());
    }

    /// Tests precedence of `client_id` over a derived tenant-based client ID.
    #[test]
    fn test_token_fetcher_builder_client_id_precedence() {
        let platform = Platform::NpLz;
        let tenant = "my-tenant";
        let client_id_override = "override";
        let client_secret = "secret";
        let tf = ManagementApiTokenFetcherBuilder::new(platform)
            .tenant_name(tenant.to_string())
            .client_id(client_id_override.to_string())
            .client_secret(client_secret.to_string())
            .build()
            .unwrap();
        assert_eq!(tf.client_id, client_id_override);
        assert_eq!(tf.client_secret, client_secret);
        assert_eq!(tf.auth_url, Platform::NpLz.endpoint_management_api_token());
    }

    /// Ensures builder returns errors if `client_id` or `client_secret` are missing.
    #[test]
    fn test_token_fetcher_builder_build_error() {
        let err = ManagementApiTokenFetcherBuilder::new(Platform::NpLz)
            .client_secret("client_secret".to_string())
            .build()
            .unwrap_err();
        assert!(matches!(err, ManagementApiTokenError::UnknownClientId));

        let err = ManagementApiTokenFetcherBuilder::new(Platform::NpLz)
            .tenant_name("tenant_name".to_string())
            .build()
            .unwrap_err();
        assert!(matches!(err, ManagementApiTokenError::UnknownClientSecret));
    }
}
