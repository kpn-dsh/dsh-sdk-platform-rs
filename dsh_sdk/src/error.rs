/// Errors for the DSH SDK
#[derive(Debug, thiserror::Error)]
pub enum DshError {
    #[error("Certificates error: {0}")]
    CertificatesError(#[from] crate::certificates::CertificatesError),
    #[error("Datastream error: {0}")]
    DatastreamError(#[from] crate::datastream::DatastreamError),
    #[error("Utils error: {0}")]
    UtilsError(#[from] crate::utils::UtilsError),
    #[error("Reqwest: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub(crate) fn report(mut err: &dyn std::error::Error) -> String {
    let mut s = format!("{}", err);
    while let Some(src) = err.source() {
        s.push_str(&format!("\n\nCaused by: {}", src));
        err = src;
    }
    s
}
