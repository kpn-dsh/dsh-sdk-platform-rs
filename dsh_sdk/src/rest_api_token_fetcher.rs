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
    /// use dsh_sdk::dsh::rest_api_client::{TokenFetcher, };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let platform = Platform::NpLz;
    ///     let client_id = platform.client_id("my-tenant");
    ///     let client_secret = "my-secret".to_string();
    ///     let token_fetcher = TokenFetcher::new(client_id, client_secret, platform.endpoint_rest_access_token());
    ///     let token = token_fetcher.get_token().await.unwrap();
    /// }
    /// ```
    pub fn new(client_id: String, client_secret: String, auth_url: String) -> Self {
        Self {
            access_token: Mutex::new(AccessToken::default()),
            fetched_at: Mutex::new(Instant::now()),
            client_id,
            client_secret,
            client: reqwest::Client::new(),
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
                let access_token = self
                    .fetch_access_token_from_server()
                    .await
                    .map_err(DshRestTokenError::FailureTokenFetch)?;
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
    /// If the request fails, it will return a `reqwest::Error` error.
    pub async fn fetch_access_token_from_server(&self) -> Result<AccessToken, reqwest::Error> {
        self.client
            .post(&self.auth_url)
            .form(&[
                ("client_id", self.client_id.as_ref()),
                ("client_secret", self.client_secret.as_ref()),
                ("grant_type", "client_credentials"),
            ])
            .send()
            .await?
            .json::<AccessToken>()
            .await
    }
}

/// Builder for the token fetcher
pub struct RestTokenFetcherBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
    platform: Platform,
    tenant_name: Option<String>,
}

impl RestTokenFetcherBuilder {
    /// Get a new instance of the ClientBuilder
    pub fn new(platform: Platform) -> Self {
        Self {
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

    /// Build the client and token fetcher
    ///
    /// This will build the client and token fetcher based on the given parameters.
    /// It will return a tuple with the client and token fetcher.
    ///
    /// ## Example
    /// ```
    /// # use dsh_sdk::dsh::rest_api_client::{ClientBuilder, Platform};
    /// let platform = Platform::NpLz;
    /// let client_id = "robot:dev-lz-dsh:my-tenant".to_string();
    /// let client_secret = "secret".to_string();
    /// let (client, token_fetcher) = ClientBuilder::new(platform)
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
        let token_fetcher = RestTokenFetcher::new(
            client_id,
            client_secret,
            self.platform.endpoint_rest_access_token().to_string(),
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
        f.debug_struct("ClientBuilder")
            .field("client_id", &self.client_id)
            .field("client_secret", &client_secret)
            .field("platform", &self.platform)
            .field("tenant_name", &self.tenant_name)
            .finish()
    }
}
