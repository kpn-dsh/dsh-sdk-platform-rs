use serde::{Deserialize, Serialize};

use super::claims::Claims;
use super::token::RestToken;
use crate::protocol_adapters::token::ProtocolTokenError;

/// Request for geting a [`RestToken`] which can be used to get a [`DataAccessToken`](crate::protocol_adapters::token::data_access_token::DataAccessToken).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct RequestRestToken {
    /// Tenant name
    tenant: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requested expiration time in seconds (in seconds since UNIX epoch)
    exp: Option<i64>,
    /// Requested claims and permissions that the [`DataAccessToken`](crate::protocol_adapters::token::data_access_token::DataAccessToken) should have
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<Claims>,
}

impl RequestRestToken {
    /// Creates a new [`RequestRestToken`] instance with full access request.
    ///
    /// # Arguments
    /// - `tenant` - The tenant name or API client name.
    ///
    /// # Returns
    /// A new [`RequestRestToken`] instance with full access.
    pub fn new(tenant: impl Into<String>) -> Self {
        Self {
            tenant: tenant.into(),
            exp: None,
            claims: None,
        }
    }

    /// Send the request to the DSH platform to get a [`RestToken`].
    ///
    /// # Arguments
    /// - `client` - The [reqwest client](reqwest::Client) to use for the request.
    /// - `api_key` - The API key to authenticate to the DSH platform.
    /// - `auth_url` - The URL of the DSH platform to send the request to (See [Platform::endpoint_protocol_rest_token](crate::Platform::endpoint_protocol_rest_token)).
    ///
    /// # Returns
    /// The [`RestToken`] if the request was successful.
    /// Otherwise a [`ProtocolTokenError`] is returned.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::RequestRestToken;
    /// use dsh_sdk::Platform;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let request = RequestRestToken::new("example_tenant");
    /// let client = reqwest::Client::new();
    /// let platform = Platform::NpLz;
    /// let token = request.send(&client, "API_KEY", platform.endpoint_protocol_rest_token()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(
        &self,
        client: &reqwest::Client,
        api_key: &str,
        auth_url: &str,
    ) -> Result<RestToken, ProtocolTokenError> {
        log::debug!("Sending request to '{}': {:?}", auth_url, self);
        let response = client
            .post(auth_url)
            .header("apikey", api_key)
            .json(self)
            .send()
            .await?;

        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(RestToken::parse(body_text)?),
            _ => Err(ProtocolTokenError::DshCall {
                url: auth_url.to_string(),
                status_code: status,
                error_body: body_text,
            }),
        }
    }

    /// Returns the tenant name or API client name.
    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    /// Sets the expiration time (in seconds since UNIX epoch)
    pub fn set_exp(mut self, exp: i64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Returns the expiration time (in seconds since UNIX epoch)
    pub fn exp(&self) -> Option<i64> {
        self.exp
    }

    /// Sets the claims
    pub fn set_claims(mut self, claims: impl Into<Claims>) -> Self {
        self.claims = Some(claims.into());
        self
    }
    /// Returns the claims
    pub fn claims(&self) -> Option<&Claims> {
        self.claims.as_ref()
    }

    /// Returns the client_id if it is set in claims
    pub fn client_id(&self) -> Option<&str> {
        self.claims.as_ref().and_then(|c| c.mqtt_token_claim().id())
    }
}

impl PartialEq for RequestRestToken {
    fn eq(&self, other: &Self) -> bool {
        // Ignore the requested expiration time, not relevant for equality
        self.tenant == other.tenant && self.claims == other.claims
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol_adapters::token::rest_token::DatastreamsMqttTokenClaim;
    use mockito::Matcher;
    use serde_json::json;

    #[test]
    fn test_rest_token_request() {
        let request = RequestRestToken::new("test-tenant");
        assert_eq!(request.tenant(), "test-tenant");
        assert_eq!(request.exp(), None);
        assert_eq!(request.claims(), None);
        let claims: Claims = DatastreamsMqttTokenClaim::new().set_exp(1).into();
        let request = request.set_exp(100).set_claims(claims.clone());
        let request = request;
        assert_eq!(request.exp(), Some(100));
        assert_eq!(request.claims(), Some(&claims))
    }

    #[tokio::test]
    async fn test_send_success() {
        let mut mockito_server = mockito::Server::new_async().await;
        let _m = mockito_server
            .mock("POST", "/protocol_auth_url")
            .match_header("apikey", "test_token")
            .match_body(Matcher::Json(json!({"tenant": "test_tenant"})))
            .with_status(200)
            .with_body("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJlbmRwb2ludCI6InRlc3RfZW5wb2ludCIsImNsYWltcyI6eyJkYXRhc3RyZWFtcy92MC9tcXR0L3Rva2VuIjp7fX19.Eh2-UBOgame_cQw5iHjc19-hRZXAPxMYlCHVCwcE8CU")
            .create();

        let client = reqwest::Client::new();
        let request = RequestRestToken::new("test_tenant");
        let result = request
            .send(
                &client,
                "test_token",
                &format!("{}/protocol_auth_url", mockito_server.url()),
            )
            .await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(token.is_valid());
    }
}
