/// Errors related to certificates
#[derive(Debug, thiserror::Error)]
pub enum CertificatesError {
    #[error("Certificates are not set")]
    NoCertificates,
    #[error("Error parsing: {0}")]
    ParseDn(String),
    #[error("Error calling: {url}, status code: {status_code}, error body: {error_body}")]
    DshCallError {
        url: String,
        status_code: reqwest::StatusCode,
        error_body: String,
    },
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Rcgen error: {0}")]
    PrivateKey(#[from] rcgen::Error),
    #[error("Invalid PEM certificate: {0}")]
    PemError(#[from] pem::PemError),
    #[error("Utils error: {0}")]
    UtilsError(#[from] crate::utils::UtilsError),
    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}
