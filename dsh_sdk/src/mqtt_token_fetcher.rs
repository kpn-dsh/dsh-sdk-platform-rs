pub use crate::protocol_adapters::token_fetcher::*;

use crate::Platform;

#[deprecated(
    since = "0.5.0",
    note = "`MqttTokenFetcher` is renamed to `ProtocolTokenFetcher`"
)]
pub struct MqttTokenFetcher;

impl MqttTokenFetcher {
    pub fn new(tenant_name: String, api_key: String, platform: Platform) -> ProtocolTokenFetcher {
        ProtocolTokenFetcher::new(tenant_name, api_key, platform)
    }

    pub fn new_with_client(
        tenant_name: String,
        api_key: String,
        platform: Platform,
        client: reqwest::Client,
    ) -> ProtocolTokenFetcher {
        ProtocolTokenFetcher::new_with_client(tenant_name, api_key, platform, client)
    }
}
