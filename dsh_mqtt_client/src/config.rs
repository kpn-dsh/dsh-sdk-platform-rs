use std::sync::Arc;

use lombok::{Getter, Setter, ToString};
use serde::Deserialize;

pub type ArcDshConfig = Arc<DshConfig>;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum DshEnv {
    Dev,
    Prod,
    Poc,
}

#[derive(Deserialize, Setter, Getter, ToString)]
pub struct DshConfig {
    pub env: DshEnv,
    rest_token_endpoint: String,
    mqtt_token_endpoint: String,
}

impl Default for DshConfig {
    fn default() -> Self {
        Self::new(DshEnv::Dev)
    }
}

impl DshConfig {
    pub fn new(env: DshEnv) -> DshConfig {
        DshConfig {
            env,
            rest_token_endpoint: Self::rest_token_endpoint(&env),
            mqtt_token_endpoint: Self::mqtt_token_endpoint(&env),
        }
    }

    fn rest_token_endpoint(env: &DshEnv) -> String {
        match env {
            DshEnv::Dev => "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token".to_string(),
            DshEnv::Prod => "https://api.dsh-prod.dsh.np.aws.kpn.com/auth/v0/token".to_string(),
            DshEnv::Poc => "https://api.poc.kpn-dsh.com/auth/v0/token".to_string(),
        }
    }

    fn mqtt_token_endpoint(env: &DshEnv) -> String {
        match env {
            DshEnv::Dev => {
                "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token".to_string()
            }
            DshEnv::Prod => {
                "https://api.dsh-prod.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token".to_owned()
            }
            DshEnv::Poc => "https://api.poc.kpn-dsh.com/datastreams/v0/mqtt/token".to_owned(),
        }
    }
}
