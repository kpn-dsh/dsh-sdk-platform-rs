//! Token fetcher for fetching Rest and Data Access tokens by an API Client authentication service.
//!
//! An API client is an organization that develops or supplies services (applications)
//! and devices (external clients) that will read data from or publish data on the public
//! data streams on the DSH platform.
//!
//! **NEVER** use this fetcher in a device or external client.
//!
//! This fetcher should be used by your API Client authentication services that delegates [`RestToken`]
//! and/or [`DataAccessToken`] to external clients.
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use tokio::sync::RwLock;

use super::data_access_token::*;
use super::rest_token::*;
use super::ProtocolTokenError;
use crate::Platform;

/// Fetcher for Rest and Data Access tokens by an API Client authentication service.
///
/// **NEVER** use this fetcher in a device or external client.
///
/// This fetcher should be used by your API Client authentication services that delegates [`RestToken`]
/// and/or [`DataAccessToken`] to external clients.
pub struct ApiClientTokenFetcher {
    api_key: String,
    platform: Platform,
    cache_rest_tokens: RwLock<HashMap<u64, RestToken>>,
    cache_data_access_tokens: RwLock<HashMap<u64, DataAccessToken>>,
    reqwest_client: reqwest::Client,
}

impl ApiClientTokenFetcher {
    /// Creates a new instance of the API client token fetcher.
    ///
    /// # Arguments
    /// - `api_key` - The API key to authenticate to DSH.
    /// - `platform` - The DSH platform to fetch the token for.
    pub fn new(api_key: impl Into<String>, platform: Platform) -> Self {
        let reqwest_client = reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .expect("Failed to create reqwest client with rustls as tls backend");
        Self {
            api_key: api_key.into(),
            platform,
            cache_rest_tokens: RwLock::new(HashMap::new()),
            cache_data_access_tokens: RwLock::new(HashMap::new()),
            reqwest_client,
        }
    }

    /// Clears the cache of all [`RestToken`]s.
    pub async fn clear_cache_rest_tokens(&self) {
        self.cache_rest_tokens.write().await.clear();
    }

    /// Clears the cache of all [`DataAccessToken`]s.
    pub async fn clear_cache_data_access_tokens(&self) {
        self.cache_data_access_tokens.write().await.clear();
    }

    /// Clears the cache of all tokens.
    pub async fn clear_cache(&self) {
        self.clear_cache_rest_tokens().await;
        self.clear_cache_data_access_tokens().await;
    }

    /// Fetches a new [`RestToken`] from the DSH platform.
    ///
    /// It will check the cache first and check if it is still valid.
    /// If not it will fetch a new [`RestToken`]
    pub async fn fetch_rest_token(
        &self,
        request: RequestRestToken,
    ) -> Result<RestToken, ProtocolTokenError> {
        // Get the tenant name from the request
        let tenant = request.tenant();
        // Get the client_id from the request, if not present use the tenant name as client_id
        let client_id = request
            .claims()
            .and_then(|claim| claim.mqtt_token_claim().id())
            .unwrap_or_else(|| request.tenant());
        let mut hasher = DefaultHasher::new();
        (tenant.to_string(), client_id.to_string()).hash(&mut hasher);
        let key = hasher.finish();

        // Check if a valid token is already in the cache (read lock for better concurency)
        let cache_read_ref = self.cache_rest_tokens.read().await;
        if let Some(token) = cache_read_ref.get(&key) {
            if token.is_valid() {
                log::trace!(
                    "Valid token found in cache for tenant '{}' client '{}': {:?}",
                    tenant,
                    client_id,
                    token
                );
                return Ok(token.clone());
            }
        }

        log::trace!(
            "No (valid) token found in cache for tenant '{}' client '{}'",
            tenant,
            client_id
        );
        // Drop the read lock and get a write lock
        drop(cache_read_ref);
        let mut cache_write_lock = self.cache_rest_tokens.write().await;

        // Get a reference to the token
        let token = cache_write_lock.entry(key).or_insert(RestToken::init());

        // Check if the token is valid (for if another thread already fetched a new token)
        if !token.is_valid() {
            *token = request
                .send(
                    &self.reqwest_client,
                    &self.api_key,
                    &self.platform.endpoint_protocol_rest_token(),
                )
                .await?
        };
        log::trace!(
            "Fetched new token for tenant '{}' client '{}': {:?}",
            tenant,
            client_id,
            token
        );
        Ok(token.clone())
    }

    /// Fetches a new [`DataAccessToken`] from the DSH platform.
    pub async fn fetch_data_access_token(
        &self,
        request: RequestDataAccessToken,
    ) -> Result<DataAccessToken, ProtocolTokenError> {
        let mut hasher = DefaultHasher::new();
        request.hash(&mut hasher);
        let key = hasher.finish();
        let client_id = request.id();
        let cache_read_ref = self.cache_data_access_tokens.read().await;
        if let Some(token) = cache_read_ref.get(&key) {
            if token.is_valid() {
                log::trace!(
                    "Valid token found in cache for client '{}': {:?}",
                    client_id,
                    token
                );
                return Ok(token.clone());
            }
        }

        log::trace!("No (valid) token found in cache for client '{}'", client_id);
        // Drop the read lock and get a write lock
        drop(cache_read_ref);
        let mut cache_write_lock = self.cache_data_access_tokens.write().await;

        // Get a reference to the token
        let token = cache_write_lock
            .entry(key)
            .or_insert(DataAccessToken::init());

        // Check if the token is valid (for if another thread already fetched a new token)
        if !token.is_valid() {
            let requested_tenant = request.tenant();
            let rest_token = self
                .fetch_rest_token(RequestRestToken::new(requested_tenant))
                .await?;
            *token = request.send(&self.reqwest_client, rest_token).await?
        };
        log::trace!("Fetched new token for client '{}': {:?}", client_id, token);
        Ok(token.clone())
    }
}

impl std::fmt::Debug for ApiClientTokenFetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClientTokenFetcher")
            .field("api_key", &"xxxxxx")
            .field("platform", &self.platform)
            .finish()
    }
}
