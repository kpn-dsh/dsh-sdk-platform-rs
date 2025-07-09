//! Access Token to authenticate to the DSH Mqtt or Http brokers
use super::ProtocolTokenError;

mod claims;
mod request;
mod token;

#[doc(inline)]
pub use claims::{Action, TopicPermission};
#[doc(inline)]
pub use request::RequestDataAccessToken;
#[doc(inline)]
pub use token::{DataAccessToken, Ports};

/// Validates if a string can be used as a client_id
///
/// DSH Allows the following as a client_id:
/// - A maximum of 64 characters
/// - Can only contain:
///     - Alphanumeric characters (a-z, A-z, 0-9)
///     - @, -, _, . and :
///
/// it will return an [ProtocolTokenError::InvalidClientId] if the client_id is invalid
/// including the reason why it is invalid
///
/// # Example
/// ```
/// # use dsh_sdk::protocol_adapters::token::data_access_token::validate_client_id;
/// // valid client id's
/// assert!(validate_client_id("client-12345").is_ok());
/// assert!(validate_client_id("ABCDEFasbcdef1234567890@-_.:").is_ok());
///
/// // invalid client id's
/// assert!(validate_client_id("client A").is_err());
/// assert!(validate_client_id("1234567890qwertyuiopasdfghjklzxcvbnmz1234567890qwertyuiopasdfghjklzxcvbnmz").is_err());
/// ```
pub fn validate_client_id(id: impl AsRef<str>) -> Result<(), ProtocolTokenError> {
    let ref_id = id.as_ref();
    if !ref_id.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '@' || c == '-' || c == '_' || c == '.' || c == ':'
    }) {
        Err(ProtocolTokenError::InvalidClientId(
            ref_id.to_string(),
            "client_id: Can only contain: Alphanumeric characters (a-z, A-z, 0-9) @, -, _, . and :",
        ))
    } else if ref_id.len() > 64 {
        // Note this works because all valid characters are ASCII and have a single byte
        Err(ProtocolTokenError::InvalidClientId(
            ref_id.to_string(),
            "Exceeded a maximum of 64 characters",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_client_id() {
        assert!(validate_client_id("ABCDEF1234567890@-_.:asbcdef").is_ok());
        assert!(validate_client_id("!").is_err());
        assert!(
            validate_client_id(
                "1234567890qwertyuiopasdfghjklzxcvbnmz1234567890qwertyuiopasdfghjklzxcvbnmz"
            )
            .is_err()
        );
        assert!(validate_client_id("client A").is_err());
        assert!(validate_client_id("client\nA").is_err());
        assert!(validate_client_id(r#"client\nA"#).is_err());
    }
}
