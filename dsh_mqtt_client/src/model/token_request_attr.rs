use serde::{Deserialize, Serialize};

use super::mqtt_model::Claims;

/// Represents information to be sent by client to retrieve a token.
#[derive(Serialize, Deserialize, Debug, Clone)]
//add getter setter functions
pub struct RetrieveTokenRequest {
    pub tenant: String,
    pub api_key: String,
    pub claims: Option<Vec<Claims>>,
    pub client_id: String,
}
