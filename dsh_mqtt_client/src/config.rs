use std::sync::Arc;

use lombok::{AllArgsConstructor, Builder, Getter, Setter, ToString};
use serde::Deserialize;

pub type ArcDshConfig = Arc<DshConfig>;

#[derive(Clone, Deserialize, Setter, Getter, ToString, Builder, AllArgsConstructor)]
pub struct DshConfig {
    pub rest_token_endpoint: String,
    pub mqtt_token_endpoint: String,
}
