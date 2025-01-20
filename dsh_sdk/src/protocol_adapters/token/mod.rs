//! Protocol Token
//!
//! `ProtocolTokenFetcher` is responsible for fetching and managing MQTT tokens for DSH.

pub mod api_client_token_fetcher;
pub mod data_access_token;
mod error;
pub mod rest_token;

#[doc(inline)]
pub use data_access_token::{Action, DataAccessToken, RequestDataAccessToken, TopicPermission};
#[doc(inline)]
pub use error::ProtocolTokenError;
#[doc(inline)]
pub use rest_token::{Claims, DatastreamsMqttTokenClaim, RequestRestToken, RestToken};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct JwtToken {
    header: String,
    payload: String,
    signature: String,
}

impl JwtToken {
    /// Extracts the header, payload and signature part of a JWT token.
    ///
    /// # Arguments
    ///
    /// * `raw_token` - The raw JWT token string.
    ///
    /// # Returns
    ///
    /// A Result containing the [JwtToken] or a [`ProtocolTokenError`].
    fn parse(raw_token: &str) -> Result<Self, ProtocolTokenError> {
        let parts: Vec<&str> = raw_token.split('.').collect();
        if parts.len() != 3 {
            return Err(ProtocolTokenError::Jwt(format!(
                "Invalid JWT token {}",
                raw_token
            )));
        }
        Ok(JwtToken {
            header: parts[0].to_string(),
            payload: parts[1].to_string(),
            signature: parts[2].to_string(),
        })
    }

    fn b64_decode_payload(&self) -> Result<Vec<u8>, ProtocolTokenError> {
        use base64::engine::general_purpose::STANDARD_NO_PAD;
        use base64::Engine;
        Ok(STANDARD_NO_PAD.decode(self.payload.as_bytes())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_jwt() {
        let raw = "header.payload.signature";
        let result = JwtToken::parse(raw).unwrap();
        assert_eq!(result.header, "header");
        assert_eq!(result.payload, "payload");
        assert_eq!(result.signature, "signature");

        let raw = "header.payload";
        let result = JwtToken::parse(raw);
        assert!(result.is_err());

        let raw = "header";
        let result = JwtToken::parse(raw);
        assert!(result.is_err());
    }
}
