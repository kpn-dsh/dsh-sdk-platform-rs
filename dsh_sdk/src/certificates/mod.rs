//! Handles DSH certificates and the bootstrap process.
//!
//! The [`Cert`] struct holds the DSH CA certificate, the DSH Kafka certificate, and
//! the corresponding private key. It provides methods to:
//! - Create Reqwest clients (async/blocking) that embed the Kafka certificate for secure connections
//! - Retrieve certificates and keys as PEM strings
//! - Generate certificate files (`ca.crt`, `client.pem`, and `client.key`) in a target directory
//!
//! # Usage Flow
//! Typically, you either:
//! 1. **Bootstrap**: Generate and sign certificates using [`Cert::from_bootstrap`] or [`Cert::from_env`],  
//!    which fetches or creates certificates at runtime.  
//! 2. **Load**: Read existing certificates from a directory using [`Cert::from_pki_config_dir`].  
//!
//! After obtaining a [`Cert`] instance, you can create HTTP clients or retrieve the raw certificate/key data.
//!
//! ## Creating Files
//! To create the `ca.crt`, `client.pem`, and `client.key` files in a desired directory, use the
//! [`Cert::to_files`] method.
//! ```no_run
//! use dsh_sdk::certificates::Cert;
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let certificates = Cert::from_env()?;
//! let directory = PathBuf::from("path/to/dir");
//! certificates.to_files(&directory)?;
//! # Ok(())
//! # }
//! ```
use std::path::PathBuf;
use std::sync::Arc;

use log::info;
use rcgen::KeyPair;
use reqwest::blocking::{Client, ClientBuilder};

#[doc(inline)]
pub use error::CertificatesError;

use crate::utils;
use crate::{VAR_KAFKA_CONFIG_HOST, VAR_TASK_ID};

mod bootstrap;
mod error;
mod pki_config_dir;

/// Holds all relevant certificates and private keys to connect to the DSH Kafka cluster and the Schema Store.
///
/// This struct includes:
/// - `dsh_ca_certificate_pem`: The CA certificate (equivalent to `ca.crt`)
/// - `dsh_client_certificate_pem`: The client (Kafka) certificate (equivalent to `client.pem`)
/// - `key_pair`: The private key used for Kafka connections (equivalent to `client.key`)
#[derive(Debug, Clone)]
pub struct Cert {
    dsh_ca_certificate_pem: String,
    dsh_client_certificate_pem: String,
    key_pair: Arc<KeyPair>,
}

impl Cert {
    /// Creates a new [`Cert`] struct from the given certificate strings and key pair.
    fn new(
        dsh_ca_certificate_pem: String,
        dsh_client_certificate_pem: String,
        key_pair: KeyPair,
    ) -> Cert {
        Self {
            dsh_ca_certificate_pem,
            dsh_client_certificate_pem,
            key_pair: Arc::new(key_pair),
        }
    }

    /// Bootstraps to DSH and signs the certificates.
    ///
    /// This fetches the DSH CA certificate, creates/signs a Kafka certificate, and generates a private key.
    ///
    /// # Recommended Approach
    /// Use [`Cert::from_env`] if you rely on environment variables injected by DSH (e.g., `KAFKA_CONFIG_HOST`,
    /// `MESOS_TASK_ID`). This allows an easier switch between Kafka Proxy, VPN connection, etc.
    ///
    /// # Arguments
    /// - `config_host`: The DSH config host where the CSR is sent.
    /// - `tenant_name`: The tenant name.
    /// - `task_id`: The running container’s task ID.
    ///
    /// # Errors
    /// Returns a [`CertificatesError`] if the bootstrap process fails (e.g., network issues or invalid inputs).
    pub fn from_bootstrap(
        config_host: &str,
        tenant_name: &str,
        task_id: &str,
    ) -> Result<Self, CertificatesError> {
        bootstrap::bootstrap(config_host, tenant_name, task_id)
    }

    /// Bootstraps to DSH and signs certificates based on environment variables injected by DSH.
    ///
    /// This method checks if `PKI_CONFIG_DIR` is set:
    /// - If it is, certificates are loaded from that directory (e.g., when using Kafka Proxy or VPN).
    /// - Otherwise, it uses `KAFKA_CONFIG_HOST`, `MESOS_TASK_ID`, and `MARATHON_APP_ID` to bootstrap
    ///   and sign certificates.
    ///
    /// # Errors
    /// Returns a [`CertificatesError::MisisngInjectedVariables`] if required environment variables are absent,
    /// or if the bootstrap operation fails for another reason.
    pub fn from_env() -> Result<Self, CertificatesError> {
        // Attempt to load from PKI_CONFIG_DIR
        if let Ok(cert) = Self::from_pki_config_dir::<std::path::PathBuf>(None) {
            Ok(cert)
        } else if let (Ok(config_host), Ok(task_id), Ok(tenant_name)) = (
            utils::get_env_var(VAR_KAFKA_CONFIG_HOST),
            utils::get_env_var(VAR_TASK_ID),
            utils::tenant_name(),
        ) {
            Self::from_bootstrap(&config_host, &tenant_name, &task_id)
        } else {
            Err(CertificatesError::MisisngInjectedVariables)
        }
    }

    /// Loads the certificates from a specified directory (or from `PKI_CONFIG_DIR` if set).
    ///
    /// Useful if certificates are already created and stored locally (e.g., Kafka Proxy, VPN usage).
    ///
    /// # Arguments
    /// - `path`: An optional path to the directory containing the certificates in PEM format.
    ///
    /// If omitted, the `PKI_CONFIG_DIR` environment variable is used.
    ///
    /// # Note
    /// - Only PEM format for certificates is supported.
    /// - Key files should be in PKCS#8 format and can be in DER or PEM.
    ///
    /// # Errors
    /// Returns a [`CertificatesError`] if files are missing, malformed, or cannot be read.
    pub fn from_pki_config_dir<P>(path: Option<P>) -> Result<Self, CertificatesError>
    where
        P: AsRef<std::path::Path>,
    {
        pki_config_dir::get_pki_certificates(path)
    }

    /// Builds an **async** Reqwest client with the DSH Kafka certificate included.
    ///
    /// This client can be used to securely fetch `datastreams.json` or connect to the Schema Registry.
    ///
    /// # Panics
    /// Panics if the certificate or private key is invalid. In practice, this should not occur if
    /// the [`Cert`] was instantiated successfully.
    pub fn reqwest_client_config(&self) -> reqwest::ClientBuilder {
        let (pem_identity, reqwest_cert) = Self::prepare_reqwest_client(
            self.dsh_kafka_certificate_pem(),
            &self.private_key_pem(),
            self.dsh_ca_certificate_pem(),
        );
        reqwest::Client::builder()
            .add_root_certificate(reqwest_cert)
            .identity(pem_identity)
            .use_rustls_tls()
    }

    /// Builds a **blocking** Reqwest client with the DSH Kafka certificate included.
    ///
    /// This client can be used to securely fetch `datastreams.json` or connect to the Schema Registry.
    ///
    /// # Panics
    /// Panics if the certificate or private key is invalid. This should not occur if
    /// the [`Cert`] was instantiated successfully.
    pub fn reqwest_blocking_client_config(&self) -> ClientBuilder {
        let (pem_identity, reqwest_cert) = Self::prepare_reqwest_client(
            self.dsh_kafka_certificate_pem(),
            &self.private_key_pem(),
            self.dsh_ca_certificate_pem(),
        );
        Client::builder()
            .add_root_certificate(reqwest_cert)
            .identity(pem_identity)
            .use_rustls_tls()
    }

    /// Returns the root CA certificate as a PEM string (equivalent to `ca.crt`).
    pub fn dsh_ca_certificate_pem(&self) -> &str {
        &self.dsh_ca_certificate_pem
    }

    /// Returns the Kafka certificate as a PEM string (equivalent to `client.pem`).
    pub fn dsh_kafka_certificate_pem(&self) -> &str {
        &self.dsh_client_certificate_pem
    }

    /// Returns the private key in PKCS#8 ASN.1 DER-encoded bytes.
    pub fn private_key_pkcs8(&self) -> Vec<u8> {
        self.key_pair.serialize_der()
    }

    /// Returns the private key as a PEM string (equivalent to `client.key`).
    pub fn private_key_pem(&self) -> String {
        self.key_pair.serialize_pem()
    }

    /// Returns the public key in PEM format.
    pub fn public_key_pem(&self) -> String {
        self.key_pair.public_key_pem()
    }

    /// Returns the public key as DER bytes.
    pub fn public_key_der(&self) -> Vec<u8> {
        self.key_pair.public_key_der()
    }

    /// Creates `ca.crt`, `client.pem`, and `client.key` files in the specified directory.
    ///
    /// This method also creates the directory if it doesn't exist.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::certificates::Cert;
    /// use std::path::PathBuf;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let certificates = Cert::from_env()?;
    /// let directory = PathBuf::from("path/to/dir");
    /// certificates.to_files(&directory)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns a [`CertificatesError`] if files cannot be created or written.
    pub fn to_files(&self, dir: &PathBuf) -> Result<(), CertificatesError> {
        std::fs::create_dir_all(dir)?;
        Self::create_file(dir.join("ca.crt"), self.dsh_ca_certificate_pem())?;
        Self::create_file(dir.join("client.pem"), self.dsh_kafka_certificate_pem())?;
        Self::create_file(dir.join("client.key"), self.private_key_pem())?;
        Ok(())
    }

    /// Internal helper to create a file with the specified contents.
    fn create_file<C: AsRef<[u8]>>(path: PathBuf, contents: C) -> Result<(), CertificatesError> {
        std::fs::write(&path, contents)?;
        info!("File created ({})", path.display());
        Ok(())
    }

    /// Creates a [`reqwest::Identity`] from the certificate and private key bytes.
    ///
    /// # Errors
    /// Returns a `reqwest::Error` if the provided bytes are invalid.
    fn create_identity(
        cert: &[u8],
        private_key: &[u8],
    ) -> Result<reqwest::Identity, reqwest::Error> {
        let mut ident = private_key.to_vec();
        ident.extend_from_slice(b"\n");
        ident.extend_from_slice(cert);
        reqwest::Identity::from_pem(&ident)
    }

    /// Internal helper to set up the [`reqwest::Identity`] and root certificate.
    ///
    /// # Panics
    /// Panics if the certificate or key is invalid, but they should already be validated
    /// during [`Cert`] construction.
    fn prepare_reqwest_client(
        kafka_certificate: &str,
        private_key: &str,
        ca_certificate: &str,
    ) -> (reqwest::Identity, reqwest::tls::Certificate) {
        let pem_identity =
            Cert::create_identity(kafka_certificate.as_bytes(), private_key.as_bytes())
                .expect("Error creating identity. The Kafka certificate or key is invalid.");

        let reqwest_cert = reqwest::tls::Certificate::from_pem(ca_certificate.as_bytes())
            .expect("Error parsing CA certificate as PEM. The certificate is invalid.");

        (pem_identity, reqwest_cert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rcgen::{generate_simple_self_signed, CertifiedKey};
    use std::sync::OnceLock;

    use openssl::pkey::PKey;

    static TEST_CERTIFICATES: OnceLock<Cert> = OnceLock::new();

    fn set_test_cert() -> Cert {
        let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];
        let CertifiedKey { cert, key_pair } =
            generate_simple_self_signed(subject_alt_names).unwrap();
        Cert::new(cert.pem(), cert.pem(), key_pair)
    }

    #[test]
    fn test_private_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey_pem_bytes = pkey.private_key_to_pem_pkcs8().unwrap();

        let key_pem = cert.private_key_pem();
        let pkey_pem = String::from_utf8_lossy(&pkey_pem_bytes);
        assert_eq!(key_pem, pkey_pem);
    }

    #[test]
    fn test_public_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(&der).unwrap();
        let pkey_pub_pem_bytes = pkey.public_key_to_pem().unwrap();

        let pub_pem = cert.public_key_pem();
        let pkey_pub_pem = String::from_utf8_lossy(&pkey_pub_pem_bytes);
        assert_eq!(pub_pem, pkey_pub_pem);
    }

    #[test]
    fn test_public_key_der() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(&der).unwrap();
        let pkey_pub_der = pkey.public_key_to_der().unwrap();

        let pub_der = cert.public_key_der();
        assert_eq!(pub_der, pkey_pub_der);
    }

    #[test]
    fn test_private_key_pkcs8() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(&der).unwrap();
        let pkey = pkey.private_key_to_pkcs8().unwrap();

        let key = cert.private_key_pkcs8();
        assert_eq!(key, pkey);
    }

    #[test]
    fn test_dsh_ca_certificate_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.dsh_ca_certificate_pem();
        assert_eq!(pem, cert.dsh_ca_certificate_pem);
    }

    #[test]
    fn test_dsh_kafka_certificate_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.dsh_kafka_certificate_pem();
        assert_eq!(pem, cert.dsh_client_certificate_pem);
    }

    #[test]
    fn test_write_files() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let dir = PathBuf::from("test_files");
        cert.to_files(&dir).unwrap();
        let dir = "test_files";
        assert!(std::path::Path::new(&format!("{}/ca.crt", dir)).exists());
        assert!(std::path::Path::new(&format!("{}/client.pem", dir)).exists());
        assert!(std::path::Path::new(&format!("{}/client.key", dir)).exists());
    }

    #[test]
    fn test_create_identity() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let identity = Cert::create_identity(
            cert.dsh_kafka_certificate_pem().as_bytes(),
            cert.private_key_pem().as_bytes(),
        );
        assert!(identity.is_ok());
    }
}
