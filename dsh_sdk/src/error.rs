//! Error types and reporting utilities for the DSH SDK.
//!
//! This module defines the primary error enum, [`DshError`], which aggregates
//! sub-errors from certificates, datastreams, and various utilities. It also
//! includes a helper function, [`report`], for generating a more readable error
//! trace by iterating over source causes.

/// Errpors defined in [`Dsh`](super::Dsh).
///
/// This enum wraps more specific errors from different parts of the SDK:
/// - [`CertificatesError`](crate::certificates::CertificatesError)
/// - [`DatastreamError`](crate::datastream::DatastreamError)
/// - [`UtilsError`](crate::utils::UtilsError)
///
/// Each variant implements `std::error::Error` and can be conveniently converted
/// from the underlying error types (via `#[from]`).
///
#[derive(Debug, thiserror::Error)]
pub enum DshError {
    /// Wraps an error originating from certificate handling.
    #[error("Certificates error: {0}")]
    CertificatesError(#[from] crate::certificates::CertificatesError),

    /// Wraps an error originating from datastream operations or configuration.
    #[error("Datastream error: {0}")]
    DatastreamError(#[from] crate::datastream::DatastreamError),

    /// Wraps an error from general utilities or environment lookups.
    #[error("Utils error: {0}")]
    UtilsError(#[from] crate::utils::UtilsError),
}

/// Generates a user-friendly error trace by traversing all `source()`
/// causes in the given error.
///
/// The returned `String` contains the primary error message, followed
/// by each causal error (if any) on separate lines, preceded by `"Caused by:"`.
///
/// This is helpful for logging or displaying the entire chain of errors.
pub(crate) fn report(mut err: &dyn std::error::Error) -> String {
    let mut s = format!("{}", err);
    while let Some(src) = err.source() {
        s.push_str(&format!("\n\nCaused by: {}", src));
        err = src;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificates::CertificatesError;

    /// Demonstrates how to construct and print `DshError` variants,
    /// as well as how to use `report` to see the full causal chain.
    #[test]
    fn test_dsh_error_and_report() {
        // Create a wrapped DshError (CertificatesError for demonstration)
        let cert_err = CertificatesError::NoCertificates;
        let dsh_err = DshError::from(cert_err);

        // Verify the display output
        let error_message = format!("{}", dsh_err);
        println!("{}", error_message);
        assert!(error_message.contains("Certificates error: Certificates are not set"));

        // Demonstrate the 'report' function
        let report_output = report(&dsh_err);
        // Should contain the same info, but also handle possible sources.
        assert!(report_output.contains("Certificates are not set"));
    }
}
