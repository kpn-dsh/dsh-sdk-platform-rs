//!
use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;

use crate::authentication_client::{DshMqttAuthenticationClient, DshRestAuthenticationClient};
use crate::config::ArcDshConfig;
use crate::error::DshError;
use crate::model::mqtt_model::{MqttToken, MqttTokenRequest};
use crate::model::rest_model::RestTokenRequest;
use crate::model::token_request_attr::RetrieveTokenRequest;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

/// A trait for providing authentication services.
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    /// Asynchronously retrieves a token based on the provided request.
    ///
    /// # Arguments
    ///
    /// * `retrieve_token_request` - The struct holds necessary request parameters for token retrieval.
    ///
    /// # Returns
    ///
    /// A Result containing the retrieved token or an error.
    async fn retrieve_token(
        &self,
        retrieve_token_request: RetrieveTokenRequest,
    ) -> Result<MqttToken, DshError>;
}

/// An adapter implementing the `AuthenticationService` trait for DSH.
pub struct DshAuthenticationServiceAdapter {
    dsh_rest_auth_client: DshRestAuthenticationClient,
    dsh_mqtt_auth_client: DshMqttAuthenticationClient,
}

#[async_trait]
impl AuthenticationService for DshAuthenticationServiceAdapter {
    async fn retrieve_token(
        &self,
        retrieve_token_request: RetrieveTokenRequest,
    ) -> Result<MqttToken, DshError> {
        let dsh_rest_auth_client = &self.dsh_rest_auth_client;
        let rest_call_result = dsh_rest_auth_client
            .retrieve_rest_token(&RestTokenRequest::from(retrieve_token_request.clone()))
            .await?;

        let dsh_mqtt_auth_client = &self.dsh_mqtt_auth_client;

        dsh_mqtt_auth_client
            .retrieve_mqtt_token(&MqttTokenRequest::new(
                rest_call_result,
                retrieve_token_request.clone(),
            ))
            .await
    }
}

/// Creates a new instance of `DshAuthenticationServiceAdapter`.
/// To include Rest Authentication client and Mqtt Authentication Client
///
/// # Arguments
///
/// * `config` - T he configuration for DSH authentication.
impl DshAuthenticationServiceAdapter {
    pub fn new(config: ArcDshConfig) -> DshAuthenticationServiceAdapter {
        let reqwest_client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .http1_only()
            .build()
            .expect("Failed to build reqwest client");

        let rest_auth_client =
            DshRestAuthenticationClient::new(config.clone(), reqwest_client.clone());

        let mqtt_auth_client = DshMqttAuthenticationClient {
            config: config.clone(),
            reqwest_client,
        };
        DshAuthenticationServiceAdapter {
            dsh_rest_auth_client: rest_auth_client,
            dsh_mqtt_auth_client: mqtt_auth_client,
        }
    }
}
