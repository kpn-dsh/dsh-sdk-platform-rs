/// Error type for the utils module
#[derive(Debug, thiserror::Error)]
pub enum UtilsError {
    #[error("Env variable {0} error: {1}")]
    EnvVarError(&'static str, std::env::VarError),
    #[error("No tenant name found")]
    NoTenantName,
    #[error("Invalid platform: {0}")]
    InvalidPlatform(String),
}
