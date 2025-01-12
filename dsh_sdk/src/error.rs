//! Error types and reporting utilities for the DSH SDK.
//!
//! This module defines the primary error enum, [`DshError`], which aggregates
//! sub-errors from certificates, datastreams, and various utilities. It also
//! includes a helper function, [`report`], for generating a readable error
//! trace.

/// The main error type for the DSH SDK.
///
/// This enum wraps more specific errors from different parts of the SDK:
/// - [`CertificatesError`](crate::certificates::CertificatesError)
/// - [`DatastreamError`](crate::datastream::DatastreamError)
/// - [`UtilsError`](crate::utils::UtilsError)
///
/// Each variant implements `std::error::Error`, and can be conveniently
/// converted from the underlying error types (using `#[from]`).
///
/// # Example
/// ```
/// use dsh_sdk::error::DshError;
/// use dsh_sdk::certificates::CertificatesError;
///
/// // Construct a DshError from a CertificatesError:
/// let cert_err = CertificatesError::NoCertificates;
/// let dsh_err = DshError::from(cert_err);
/// println!("{}", dsh_err);
/// ```
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
/// The returned `String` contains the primary error message followed
/// by each causal error on separate lines, prefixed by `"Caused by:"`.
///
/// This can be helpful for logging or displaying a chain of errors
/// when debugging.
///
/// # Example
/// ```
/// use std::fmt;
/// use dsh_sdk::error::{DshError, report};
/// # // A simple dummy error to simulate nested error sources:
/// #[derive(Debug)]
/// struct DummyError;
/// impl fmt::Display for DummyError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "Dummy error occurred")
///     }
/// }
/// impl std::error::Error for DummyError {}
///
/// let nested_err = DshError::UtilsError(Box::new(DummyError).into());
/// let report_str = report(&nested_err);
/// assert!(report_str.contains("Dummy error occurred"));
/// println!("{}", report_str);
/// ```
pub(crate) fn report(mut err: &dyn std::error::Error) -> String {
    let mut s = format!("{}", err);
    while let Some(src) = err.source() {
        s.push_str(&format!("\n\nCaused by: {}", src));
        err = src;
    }
    s
}
