//! Token fetcher for fetching Rest and Data Access tokens by an API Client authentication service.
//!
//! An API client is an organization that develops or supplies services (applications)
//! and devices (external clients) that will read data from or publish data on the public
//! data streams on the DSH platform.
//!
//! # Note
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
/// # Note
/// **NEVER** implement this fetcher in a device or external client.
///
/// This fetcher should be used by your API Client authentication services that delegates [`RestToken`]
/// and/or [`DataAccessToken`] to external clients.
pub struct ApiClientTokenFetcher {
    api_key: String,
    auth_url: String,
    cache_rest_tokens: RwLock<HashMap<u64, RestToken>>,
    cache_data_access_tokens: RwLock<HashMap<u64, DataAccessToken>>,
    reqwest_client: reqwest::Client,
}

impl ApiClientTokenFetcher {
    /// Creates a new instance of the API client token fetcher.
    ///
    /// # Note
    /// **NEVER** implement this fetcher in a device or external client.
    ///
    /// # Arguments
    /// - `api_key` - The API key to authenticate to DSH.
    /// - `platform` - The DSH [`Platform`] to fetch the token for.
    ///
    /// # Returns
    /// A new instance of the API client token fetcher.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
    /// use dsh_sdk::Platform;
    ///
    /// // Get the API key from the environment variable
    /// let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");
    ///
    /// // Create a token fetcher with the API key and platform
    /// let token_fetcher = ApiClientTokenFetcher::new(api_key, Platform::NpLz);
    /// ```
    pub fn new(api_key: impl Into<String>, platform: Platform) -> Self {
        let reqwest_client = reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .expect("Failed to create reqwest client with rustls as tls backend");
        Self {
            api_key: api_key.into(),
            auth_url: platform.endpoint_protocol_rest_token().to_string(),
            cache_rest_tokens: RwLock::new(HashMap::new()),
            cache_data_access_tokens: RwLock::new(HashMap::new()),
            reqwest_client,
        }
    }

    /// Creates a new instance of the API client token fetcher with custom [reqwest::Client].
    ///
    /// # Note
    /// **NEVER** implement this fetcher in a device or external client.
    ///
    /// # Arguments
    /// - `api_key` - The API key to authenticate to DSH.
    /// - `platform` - The DSH [`Platform`] to fetch the token for.
    ///
    /// # Returns
    /// A new instance of the API client token fetcher.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
    /// use dsh_sdk::Platform;
    ///
    /// // Get the API key from the environment variable
    /// let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");
    ///
    /// // Create a token fetcher with the API key and platform
    /// let token_fetcher = ApiClientTokenFetcher::new(api_key, Platform::NpLz);
    /// ```
    pub fn new_with_client(
        api_key: impl Into<String>,
        platform: Platform,
        client: reqwest::Client,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            auth_url: platform.endpoint_protocol_rest_token().to_string(),
            cache_rest_tokens: RwLock::new(HashMap::new()),
            cache_data_access_tokens: RwLock::new(HashMap::new()),
            reqwest_client: client,
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
    /// # Arguments
    /// - `request` - The [`RequestRestToken`] to fetch the token.
    ///
    /// # Returns
    /// The [`RestToken`] fetched from the Cache or from the DSH platform.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::rest_token::RequestRestToken;
    /// # use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
    /// # use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");
    /// # let token_fetcher = ApiClientTokenFetcher::new(api_key, Platform::NpLz);
    /// // Create a token request
    /// let request = RequestRestToken::new("example-tenant");
    ///
    /// // Fetch token
    /// let token = token_fetcher.fetch_rest_token(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_rest_token(
        &self,
        request: RequestRestToken,
    ) -> Result<RestToken, ProtocolTokenError> {
        let token = request
            .send(&self.reqwest_client, &self.api_key, &self.auth_url)
            .await;
        log::trace!(
            "Fetched new token for tenant '{}' client '{}': {:?}",
            request.tenant(),
            request.client_id().unwrap_or(request.tenant()),
            token,
        );
        token
    }

    /// Get a [`RestToken`] from Cache if valid or fetch a new one from the DSH platform.
    ///
    /// It will check the cache first and check if it is still valid.
    /// If not it will fetch a new [`RestToken`]
    ///
    /// # Arguments
    /// - `request` - The [`RequestRestToken`] to fetch the token.
    ///
    /// # Returns
    /// The [`RestToken`] fetched from the Cache or from the DSH platform.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::rest_token::RequestRestToken;
    /// # use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
    /// # use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");
    /// # let token_fetcher = ApiClientTokenFetcher::new(api_key, Platform::NpLz);
    /// // Create a token request
    /// let request = RequestRestToken::new("example-tenant");
    ///
    /// // Get or fetch token
    /// let token = token_fetcher.get_or_fetch_rest_token(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_or_fetch_rest_token(
        &self,
        request: RequestRestToken,
    ) -> Result<RestToken, ProtocolTokenError> {
        // Get the tenant name from the request
        let tenant = request.tenant();
        // Get the client_id from the request, if not present use the tenant name as client_id
        let client_id = request
            .claims()
            .and_then(|claim| claim.mqtt_token_claim().id())
            .unwrap_or_else(|| tenant);

        let key = generate_cache_key((tenant, client_id));

        // Check if a valid token is already in the cache with a read lock
        if let Some(token) = self.get_valid_cached_rest_token(key).await {
            return Ok(token);
        }

        let mut cache_write_lock = self.cache_rest_tokens.write().await;

        // Get an entry in the cache
        let token = cache_write_lock.entry(key).or_insert(RestToken::init());

        // Check if the token is valid (for if another thread already fetched a new token)
        if !token.is_valid() {
            *token = self.fetch_rest_token(request).await?;
        };

        Ok(token.clone())
    }

    /// Fetches a new [`DataAccessToken`] from the DSH platform.
    ///
    /// # Arguments
    /// - `request` - The [`RequestDataAccessToken`] to fetch the token.
    ///
    /// # Returns
    /// The [`DataAccessToken`] fetched from the Cache or from the DSH platform.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::data_access_token::RequestDataAccessToken;
    /// # use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
    /// # use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");
    /// // Create a token fetcher with the API key and platform
    /// let token_fetcher = ApiClientTokenFetcher::new(api_key, Platform::NpLz);
    ///
    /// // Create a token request
    /// let request = RequestDataAccessToken::new("example-tenant", "external-client-id");
    ///
    /// // Fetch token
    /// let token = token_fetcher.fetch_data_access_token(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_data_access_token(
        &self,
        request: RequestDataAccessToken,
    ) -> Result<DataAccessToken, ProtocolTokenError> {
        let rest_token = self
            .get_or_fetch_rest_token(RequestRestToken::new(request.tenant()))
            .await?;
        let token = request.send(&self.reqwest_client, rest_token).await?;
        log::trace!(
            "Fetched new token for tenant '{}' client '{}': {:?}",
            request.tenant(),
            request.id(),
            token,
        );
        Ok(token)
    }

    /// Fetches a new [`DataAccessToken`] from the DSH platform and caches it.
    ///
    /// It will check the cache first and check if it is still valid.
    /// If not it will fetch a new [`DataAccessToken`]
    ///
    /// # Arguments
    /// - `request` - The [`RequestDataAccessToken`] to fetch the token.
    ///
    /// # Returns
    /// The [`DataAccessToken`] fetched from the Cache or from the DSH platform.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::data_access_token::RequestDataAccessToken;
    /// # use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
    /// # use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let api_key = std::env::var("API_KEY").expect("API_KEY env variable is not set");
    /// // Create a token fetcher with the API key and platform
    /// let token_fetcher = ApiClientTokenFetcher::new(api_key, Platform::NpLz);
    ///
    /// // Create a token request
    /// let request = RequestDataAccessToken::new("example-tenant", "external-client-id");
    ///
    /// // Get or fetch token
    /// let token = token_fetcher.get_or_fetch_data_access_token(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_or_fetch_data_access_token(
        &self,
        request: RequestDataAccessToken,
    ) -> Result<DataAccessToken, ProtocolTokenError> {
        let key = generate_cache_key(&request);

        if let Some(token) = self.get_valid_cached_data_access_token(key).await {
            return Ok(token);
        }

        let mut cache_write_lock = self.cache_data_access_tokens.write().await;

        // Get a reference to the token
        let token = cache_write_lock
            .entry(key)
            .or_insert(DataAccessToken::init());

        // Check if the token is valid (for if another thread already fetched a new token)
        if !token.is_valid() {
            *token = self.fetch_data_access_token(request).await?;
        };
        Ok(token.clone())
    }

    /// Attempts to retrieve a valid cached token with a read lock.
    async fn get_valid_cached_rest_token(&self, key: u64) -> Option<RestToken> {
        if let Some(token) = self.cache_rest_tokens.read().await.get(&key) {
            if token.is_valid() {
                log::trace!(
                    "Valid RestToken found in cache for tenant '{}' client '{}': {:?}",
                    token.tenant_id(),
                    token.client_id().unwrap_or(token.tenant_id()),
                    token
                );
                Some(token.clone())
            } else {
                log::trace!(
                    "Invalid RestToken found in cache for tenant '{}' client '{}': {:?}",
                    token.tenant_id(),
                    token.client_id().unwrap_or(token.tenant_id()),
                    token
                );
                None
            }
        } else {
            log::trace!("No RestToken found in cache");
            None
        }
    }

    /// Attempts to retrieve a valid cached token with a read lock.
    async fn get_valid_cached_data_access_token(&self, key: u64) -> Option<DataAccessToken> {
        if let Some(token) = self.cache_data_access_tokens.read().await.get(&key) {
            if token.is_valid() {
                log::trace!(
                    "Valid DataAccessToken found in cache for client '{}': {:?}",
                    token.client_id(),
                    token
                );
                return Some(token.clone());
            } else {
                log::trace!(
                    "Invalid DataAccessToken found in cache for client '{}': {:?}",
                    token.client_id(),
                    token
                );
            }
        } else {
            log::trace!("No DataAccessToken found in cache");
        }
        None
    }
}

impl std::fmt::Debug for ApiClientTokenFetcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiClientTokenFetcher")
            .field("api_key", &"xxxxxx")
            .field("auth_url", &self.auth_url)
            .finish()
    }
}

/// Hashes the key and returns the hash.
fn generate_cache_key(key: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{DataAccessToken, RequestDataAccessToken, RequestRestToken, RestToken};

    // create a valid fetcher with dummy tokens
    fn create_valid_fetcher() -> ApiClientTokenFetcher {
        let rest_token = RestToken::parse("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJnZW4iOjEsImVuZHBvaW50IjoiZHVtbXlfZW5kcG9pbnQiLCJpc3MiOiJTdHJpbmciLCJjbGFpbXMiOlt7InJlc291cmNlIjoiZHVtbXkiLCJhY3Rpb24iOiJwdXNoIn1dLCJleHAiOjIxNDc0ODM2NDcsImNsaWVudC1pZCI6ImR1bW15X3RlbmFudCIsImlhdCI6MCwidGVuYW50LWlkIjoiZHVtbXlfdGVuYW50In0.SbePw_EmLrkiSfk5XykLosqOoFb0xC_QE4A8283rFfY").unwrap();
        let data_access_token = DataAccessToken::parse("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiIxIiwiZ2VuIjoxLCJleHAiOjIxNDc0ODM2NDcsImlhdCI6MjE0NzQ4MzY0NywiZW5kcG9pbnQiOiJkdW1teV9lbmRwb2ludCIsInBvcnRzIjp7Im1xdHRzIjpbODg4M10sIm1xdHR3c3MiOls0NDMsODQ0M119LCJ0ZW5hbnQtaWQiOiJkdW1teV90ZW5hbnQiLCJjbGllbnQtaWQiOiJEdW1teS1jbGllbnQtaWQiLCJjbGFpbXMiOlt7ImFjdGlvbiI6InN1YnNjcmliZSIsInJlc291cmNlIjp7InR5cGUiOiJ0b3BpYyIsInByZWZpeCI6Ii90dCIsInN0cmVhbSI6ImR1bW1teSIsInRvcGljIjoiL2R1bW15LyMifX1dfQ.651w8PULFURETQoaKVyKSTE6qghxKfLSm_oODzFU1mM").unwrap();

        let mut rest_cache = HashMap::new();
        rest_cache.insert(
            generate_cache_key(("dummy_tenant", "dummy_tenant")),
            rest_token,
        );
        let mut data_cache = HashMap::new();
        data_cache.insert(
            generate_cache_key(RequestDataAccessToken::new(
                "dummy_tenant",
                "Dummy-client-id",
            )),
            data_access_token,
        );
        ApiClientTokenFetcher {
            api_key: "abc123".to_string(),
            auth_url: "dummy_auth_url".to_string(),
            cache_rest_tokens: RwLock::new(rest_cache),
            cache_data_access_tokens: RwLock::new(data_cache),
            reqwest_client: reqwest::Client::new(),
        }
    }

    #[tokio::test]
    async fn test_api_client_token_fetcher_new() {
        let rest_api_key = "test_api_key".to_string();
        let platform = Platform::NpLz;

        let fetcher = ApiClientTokenFetcher::new(rest_api_key, platform);

        assert!(fetcher.cache_rest_tokens.read().await.is_empty());
        assert!(fetcher.cache_data_access_tokens.read().await.is_empty());
        assert_eq!(fetcher.api_key, "test_api_key".to_string());
        assert_eq!(
            fetcher.auth_url,
            Platform::NpLz.endpoint_protocol_rest_token()
        );
    }

    #[tokio::test]
    async fn test_api_client_token_fetcher_new_with_client() {
        let rest_api_key = "test_api_key".to_string();
        let platform = Platform::NpLz;

        let client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
        let fetcher = ApiClientTokenFetcher::new_with_client(rest_api_key, platform, client);

        assert!(fetcher.cache_rest_tokens.read().await.is_empty());
        assert!(fetcher.cache_data_access_tokens.read().await.is_empty());
        assert_eq!(fetcher.api_key, "test_api_key".to_string());
        assert_eq!(
            fetcher.auth_url,
            Platform::NpLz.endpoint_protocol_rest_token()
        );
    }
    #[tokio::test]
    async fn test_fetch_new_rest_token() {
        let mut mockito_server = mockito::Server::new_async().await;
        let raw_rest_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJnZW4iOjEsImVuZHBvaW50IjoidGVzdF9lbmRwb2ludCIsImlzcyI6IlN0cmluZyIsImNsYWltcyI6W3sicmVzb3VyY2UiOiJ0ZXN0IiwiYWN0aW9uIjoicHVzaCJ9XSwiZXhwIjoxLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImlhdCI6MCwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQifQ.WCf03qyxV1NwxXpzTYF7SyJYwB3uAkQZ7u-TVrDRJgE";
        let _m = mockito_server
            .mock("POST", "/rest_auth_url")
            .with_status(200)
            .with_body(raw_rest_token)
            .create_async()
            .await;
        println!("server url: {}", mockito_server.url());
        let client = reqwest::Client::new();

        let fetcher = ApiClientTokenFetcher {
            api_key: "test_api_key".to_string(),
            auth_url: format!("{}{}", mockito_server.url(), "/rest_auth_url"),
            cache_data_access_tokens: RwLock::new(HashMap::new()),
            cache_rest_tokens: RwLock::new(HashMap::new()),
            reqwest_client: client,
        };

        let request = RequestRestToken::new("test_tenant");

        let result = fetcher.fetch_rest_token(request).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        let rest_token = result.unwrap();
        assert_eq!(rest_token.exp(), 1);
        assert_eq!(rest_token.gen(), 1);
        assert_eq!(rest_token.endpoint(), "test_endpoint");
        assert_eq!(rest_token.iss(), "String");
        assert_eq!(rest_token.raw_token(), raw_rest_token);
    }

    #[tokio::test]
    async fn test_fetch_new_data_access_token() {
        let mut opt: mockito::ServerOpts = mockito::ServerOpts::default();
        opt.port = 7999;
        let mut mockito_server = mockito::Server::new_with_opts_async(opt).await;
        let raw_rest_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJlbmRwb2ludCI6Imh0dHA6Ly8xMjcuMC4wLjE6Nzk5OSIsImNsYWltcyI6eyJkYXRhc3RyZWFtcy92MC9tcXR0L3Rva2VuIjp7fX19.j5ekqMiWyBhJyRQE_aARFS9mQJiN7S2rpKTsn3rZ5lQ";
        let raw_access_token =  "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywiaWF0IjoyMTQ3NDgzNjQ3LCJlbmRwb2ludCI6InRlc3RfZW5kcG9pbnQiLCJwb3J0cyI6eyJtcXR0cyI6Wzg4ODNdLCJtcXR0d3NzIjpbNDQzLDg0NDNdfSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImNsYWltcyI6W3siYWN0aW9uIjoic3Vic2NyaWJlIiwicmVzb3VyY2UiOnsidHlwZSI6InRvcGljIiwicHJlZml4IjoiL3R0Iiwic3RyZWFtIjoidGVzdCIsInRvcGljIjoiL3Rlc3QvIyJ9fV19.LwYIMIX39J502TDqpEqH5T2Rlj-HczeT3WLfs5Do3B0";
        let _m = mockito_server
            .mock("POST", "/rest_auth_url")
            .with_status(200)
            .with_body(raw_rest_token)
            .create_async()
            .await;
        let _m2 = mockito_server
            .mock("POST", "/datastreams/v0/mqtt/token")
            .with_status(200)
            .with_body(raw_access_token)
            .create();

        println!("server url: {}", mockito_server.url());
        let client = reqwest::Client::new();

        let fetcher = ApiClientTokenFetcher {
            api_key: "test_api_key".to_string(),
            auth_url: format!("{}{}", mockito_server.url(), "/rest_auth_url"),
            cache_data_access_tokens: RwLock::new(HashMap::new()),
            cache_rest_tokens: RwLock::new(HashMap::new()),
            reqwest_client: client,
        };

        let request = RequestDataAccessToken::new("test_tenant", "test_client");
        let result = fetcher.fetch_data_access_token(request).await;

        println!("{:?}", result);
        assert!(result.is_ok());
        let acces_token = result.unwrap();
        assert_eq!(acces_token.exp(), 2147483647);
        assert_eq!(acces_token.gen(), 1);
        assert_eq!(acces_token.endpoint(), "test_endpoint");
        assert_eq!(acces_token.iss(), "String");
        assert_eq!(acces_token.client_id(), "test_client");
        assert_eq!(acces_token.tenant_id(), "test_tenant");
        assert_eq!(acces_token.raw_token(), raw_access_token);
    }

    #[tokio::test]
    async fn test_get_cached_rest_token() {
        let fetcher = create_valid_fetcher();

        let request = RequestRestToken::new("dummy_tenant");
        let token = fetcher.get_or_fetch_rest_token(request).await.unwrap();
        assert_eq!(token.tenant_id(), "dummy_tenant");
        assert_eq!(token.client_id(), None);

        let request = RequestRestToken::new("not_in_cache");
        let token = fetcher.get_or_fetch_rest_token(request).await;
        assert!(token.is_err());
    }

    #[tokio::test]
    async fn test_get_cached_data_access_token() {
        let fetcher = create_valid_fetcher();

        let request = RequestDataAccessToken::new("dummy_tenant", "Dummy-client-id");
        let token = fetcher
            .get_or_fetch_data_access_token(request)
            .await
            .unwrap();
        assert_eq!(token.tenant_id(), "dummy_tenant");
        assert_eq!(token.client_id(), "Dummy-client-id");

        let request = RequestDataAccessToken::new("dummy_tenant", "not_in_cache");
        let token = fetcher.get_or_fetch_data_access_token(request).await;
        assert!(token.is_err());
    }

    #[tokio::test]
    async fn test_clear_cache_rest_tokens() {
        let fetcher = create_valid_fetcher();
        assert!(!fetcher.cache_rest_tokens.read().await.is_empty());
        fetcher.clear_cache_rest_tokens().await;
        assert!(fetcher.cache_rest_tokens.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_clear_cache_data_access_tokens() {
        let fetcher = create_valid_fetcher();
        assert!(!fetcher.cache_data_access_tokens.read().await.is_empty());
        fetcher.clear_cache_data_access_tokens().await;
        assert!(fetcher.cache_data_access_tokens.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let fetcher = create_valid_fetcher();
        assert!(!fetcher.cache_rest_tokens.read().await.is_empty());
        assert!(!fetcher.cache_data_access_tokens.read().await.is_empty());
        fetcher.clear_cache().await;
        assert!(fetcher.cache_rest_tokens.read().await.is_empty());
        assert!(fetcher.cache_data_access_tokens.read().await.is_empty());
    }

    #[test]
    fn test_debug_client_fetcher() {
        let client = create_valid_fetcher();
        let debug = format!("{:?}", client);
        assert_eq!(
            debug,
            "ApiClientTokenFetcher { api_key: \"xxxxxx\", auth_url: \"dummy_auth_url\" }"
        );
    }

    #[test]
    fn test_generate_cache_key() {
        let key = generate_cache_key(("test_tenant", "test_client"));
        assert_eq!(key, 17569805883005093029);
    }
}
