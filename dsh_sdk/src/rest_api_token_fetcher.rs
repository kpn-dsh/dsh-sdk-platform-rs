/// Fetch and store access tokens to be used in the DSH Rest API client
///
/// This struct will fetch and store access tokens to be used in the DSH Rest API client.
/// It will automatically fetch a new token if the current token is not valid.
pub struct RestTokenFetcher;

impl RestTokenFetcher {
    /// Create a new instance of the token fetcher
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: String,
    ) -> crate::ManagementApiTokenFetcher {
        crate::ManagementApiTokenFetcher::new_with_client(
            client_id,
            client_secret,
            auth_url,
            reqwest::Client::default(),
        )
    }

    /// Create a new instance of the token fetcher with custom reqwest client
    pub fn new_with_client(
        client_id: String,
        client_secret: String,
        auth_url: String,
        client: reqwest::Client,
    ) -> crate::ManagementApiTokenFetcher {
        crate::ManagementApiTokenFetcher::new_with_client(
            client_id,
            client_secret,
            auth_url,
            client,
        )
    }
}

/// Builder for the token fetcher
pub struct RestTokenFetcherBuilder;

impl RestTokenFetcherBuilder {
    /// Get a new instance of the ClientBuilder
    ///
    /// # Arguments
    /// * `platform` - The target platform to use for the token fetcher
    pub fn new(platform: crate::Platform) -> crate::ManagementApiTokenFetcherBuilder {
        crate::ManagementApiTokenFetcherBuilder::new(platform)
    }
}
