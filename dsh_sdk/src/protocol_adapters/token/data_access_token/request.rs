use serde::{Deserialize, Serialize};

use super::claims::TopicPermission;
use super::token::DataAccessToken;
use crate::protocol_adapters::token::rest_token::RestToken;
use crate::protocol_adapters::token::ProtocolTokenError;
use crate::utils::ensure_https_prefix;

/// Request for geting a [DataAccessToken] which can be used to authenticate to the DSH Mqtt or Http brokers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestDataAccessToken {
    /// Tenant name
    tenant: String,
    /// Unique client ID that must be used when connecting to the broker
    id: String,
    /// Requested expiration time (in seconds since UNIX epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<i64>,
    /// Optional list of topic permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    claims: Option<Vec<TopicPermission>>,
    /// DSH Client Claims optional field for commiumicating between external clients and DSH
    #[serde(skip_serializing_if = "Option::is_none")]
    dshclc: Option<serde_json::Value>,
}

impl RequestDataAccessToken {
    ///
    /// client_id: Has a maximum of 64 characters
    ///     Can only contain:
    ///     haracters (a-z, A-z, 0-9)
    ///     @, -, _, . and :
    pub fn new(tenant: impl Into<String>, client_id: impl Into<String>) -> Self {
        Self {
            tenant: tenant.into(),
            id: client_id.into(),
            exp: None,
            claims: None,
            dshclc: None,
        }
    }

    /// Returns the set tenant name
    pub fn tenant(&self) -> &str {
        &self.tenant
    }

    /// Returns the set client ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Set the requested expiration time for the token.
    pub fn set_exp(mut self, exp: i64) -> Self {
        self.exp = Some(exp);
        self
    }

    /// Returns the requested expiration time for the token.
    pub fn exp(&self) -> Option<i64> {
        self.exp
    }

    /// Set a list of [`TopicPermission`] for the token.
    pub fn set_claims(mut self, claims: Vec<TopicPermission>) -> Self {
        self.claims = Some(claims);
        self
    }

    /// Extend the list of [`TopicPermission`] for the token.
    pub fn extend_claims(mut self, claims: impl Iterator<Item = TopicPermission>) -> Self {
        self.claims.get_or_insert_with(Vec::new).extend(claims);
        self
    }

    /// Returns the list of [`TopicPermission`] for the token.
    pub fn claims(&self) -> Option<&Vec<TopicPermission>> {
        self.claims.as_ref()
    }

    /// Set the DSH Client Claims.
    ///
    /// This field is optional and can be used to communicate between external clients and the API client authentication service.
    pub fn set_dshclc(mut self, dshclc: impl Into<serde_json::Value>) -> Self {
        self.dshclc = Some(dshclc.into());
        self
    }

    /// Returns the DSH Client Claims.
    pub fn dshclc(&self) -> Option<&serde_json::Value> {
        self.dshclc.as_ref()
    }

    /// Send the request to the DSH platform to get a [`DataAccessToken`].
    ///
    /// # Arguments
    /// - `client` - The reqwest client to use for the request.
    /// - `rest_token` - The rest token to use for the request.
    ///
    /// # Returns
    /// The [`DataAccessToken`] if the request was successful.
    /// Otherwise a [`ProtocolTokenError`] is returned.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::protocol_adapters::token::data_access_token::RequestDataAccessToken;
    /// use dsh_sdk::protocol_adapters::token::rest_token::RestToken;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = reqwest::Client::new();
    /// let rest_token = RestToken::parse("valid.jwt.token")?;
    /// let request = RequestDataAccessToken::new("example_tenant", "Example-client-id");
    /// let token = request.send(&client, rest_token).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(
        &self,
        client: &reqwest::Client,
        rest_token: RestToken,
    ) -> Result<DataAccessToken, ProtocolTokenError> {
        super::validate_client_id(&self.id)?;

        let auth_url = ensure_https_prefix(format!(
            "{}/datastreams/v0/mqtt/token",
            rest_token.endpoint(),
        ));
        log::debug!("Sending request to '{}': {:?}", auth_url, self);
        let response = client
            .post(&auth_url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", rest_token.raw_token()),
            )
            .json(self)
            .send()
            .await?;
        let status = response.status();
        let body_text = response.text().await?;
        match status {
            reqwest::StatusCode::OK => Ok(DataAccessToken::parse(body_text)?),
            _ => Err(ProtocolTokenError::DshCall {
                url: auth_url,
                status_code: status,
                error_body: body_text,
            }),
        }
    }
}

impl PartialEq for RequestDataAccessToken {
    fn eq(&self, other: &Self) -> bool {
        // Ignore the exp field
        self.tenant == other.tenant
            && self.id == other.id
            && self.claims == other.claims
            && self.dshclc == other.dshclc
    }
}

impl std::hash::Hash for RequestDataAccessToken {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Ignore the exp field
        self.tenant.hash(state);
        self.id.hash(state);
        self.claims.hash(state);
        self.dshclc.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_request_new() {
        let request = RequestDataAccessToken::new("test_tenant", "test_id");
        assert_eq!(request.tenant, "test_tenant");
        assert_eq!(request.id, "test_id");
        assert_eq!(request.exp, None);
        assert_eq!(request.claims, None);
        assert_eq!(request.dshclc, None);
    }

    #[tokio::test]
    async fn test_send_success() {
        let mut opt: mockito::ServerOpts = mockito::ServerOpts::default();
        opt.port = 7998;
        let mut mockito_server = mockito::Server::new_with_opts_async(opt).await;
        let rest_token = RestToken::parse("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJlbmRwb2ludCI6Imh0dHA6Ly8xMjcuMC4wLjE6Nzk5OCIsImNsYWltcyI6eyJkYXRhc3RyZWFtcy92MC9tcXR0L3Rva2VuIjp7fX19.NsCVyQ8Cmp1N6QmFs1n8EgD0HJDC6zZaOxW_6xu4m10").unwrap();
        let _m = mockito_server
            .mock("POST", "/datastreams/v0/mqtt/token")
            .match_header("Authorization", "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJlbmRwb2ludCI6Imh0dHA6Ly8xMjcuMC4wLjE6Nzk5OCIsImNsYWltcyI6eyJkYXRhc3RyZWFtcy92MC9tcXR0L3Rva2VuIjp7fX19.NsCVyQ8Cmp1N6QmFs1n8EgD0HJDC6zZaOxW_6xu4m10") 
            .with_status(200)
            .with_body("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJTdHJpbmciLCJnZW4iOjEsImV4cCI6MjE0NzQ4MzY0NywiaWF0IjoyMTQ3NDgzNjQ3LCJlbmRwb2ludCI6InRlc3RfZW5kcG9pbnQiLCJwb3J0cyI6eyJtcXR0cyI6Wzg4ODNdLCJtcXR0d3NzIjpbNDQzLDg0NDNdfSwidGVuYW50LWlkIjoidGVzdF90ZW5hbnQiLCJjbGllbnQtaWQiOiJ0ZXN0X2NsaWVudCIsImNsYWltcyI6W3siYWN0aW9uIjoic3Vic2NyaWJlIiwicmVzb3VyY2UiOnsidHlwZSI6InRvcGljIiwicHJlZml4IjoiL3R0Iiwic3RyZWFtIjoidGVzdCIsInRvcGljIjoiL3Rlc3QvIyJ9fV19.LwYIMIX39J502TDqpEqH5T2Rlj-HczeT3WLfs5Do3B0")
            .create();

        let client = reqwest::Client::new();
        let request = RequestDataAccessToken::new("test_tenant", "test_client");
        let result = request.send(&client, rest_token).await;

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(token.is_valid());
    }
}
