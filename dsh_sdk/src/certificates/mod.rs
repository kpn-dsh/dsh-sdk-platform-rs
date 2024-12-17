//! This module holds the certificate struct and its methods.
//!
//! The certificate struct holds the DSH CA certificate, the DSH Kafka certificate and
//! the private key. It also has methods to create a reqwest client with the DSH Kafka
//! certificate included and to retrieve the certificates and keys as PEM strings. Also
//! it is possible to create the ca.crt, client.pem, and client.key files in a desired
//! directory.
//!
//! ## Create files
//!
//! To create the ca.crt, client.pem, and client.key files in a desired directory, use the
//! `to_files` method.
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
//!
//! ## Reqwest Client
//! With this request client we can retrieve datastreams.json and connect to Schema Registry.
use std::path::PathBuf;
use std::sync::Arc;

use log::{info, warn};
use rcgen::KeyPair;
use reqwest::blocking::{Client, ClientBuilder};

use crate::error::DshError;
use crate::utils;
use crate::{DEFAULT_CONFIG_HOST, VAR_KAFKA_CONFIG_HOST, VAR_PKI_CONFIG_DIR, VAR_TASK_ID};

#[cfg(feature = "bootstrap")]
mod bootstrap;
#[cfg(feature = "bootstrap")]
mod pki_config_dir;

/// Hold all relevant certificates and keys to connect to DSH Kafka Cluster and Schema Store.
#[derive(Debug, Clone)]
pub struct Cert {
    dsh_ca_certificate_pem: String,
    dsh_client_certificate_pem: String,
    key_pair: Arc<KeyPair>,
}

impl Cert {
    /// Create new `Cert` struct
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

    /// Bootstrap to DSH and sign the certificates.
    ///
    /// This method will get DSH CA certificate, sign the Kafka certificate and generate a private key.
    ///
    /// ## Recommended
    /// Use [Cert::from_env] to get the certificates and keys. As this method will check based on the injected environment variables by DSH.
    /// This method also allows you to easily switch between Kafka Proxy or VPN connection, based on `PKI_CONFIG_DIR` environment variable.
    ///
    /// ## Arguments
    /// * `config_host` - The DSH config host where the CSR can be send to. (default: `"https://pikachu.dsh.marathon.mesos:4443"`)
    /// * `tenant_name` - The tenant name.
    /// * `task_id` - The task id of running container.
    #[cfg(feature = "bootstrap")]
    pub fn from_bootstrap(
        config_host: &str,
        tenant_name: &str,
        task_id: &str,
    ) -> Result<Self, DshError> {
        bootstrap::bootstrap(config_host, tenant_name, task_id)
    }

    /// Bootstrap to DSH and sign the certificates based on the injected environment variables by DSH.
    ///
    /// This method will first check if `PKI_CONFIG_DIR` environment variable is set. If set, it will use the certificates from the directory.
    /// This is usefull when you want to use Kafka Proxy, VPN or when a different process that already created the certificates. More info at [CONNECT_PROXY_VPN_LOCAL.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/CONNECT_PROXY_VPN_LOCAL.md).
    ///
    /// Else it will check `KAFKA_CONFIG_HOST`, `MESOS_TASK_ID` and `MARATHON_APP_ID` environment variables to bootstrap to DSH and sign the certificates.
    /// These environment variables are injected by DSH.
    #[cfg(feature = "bootstrap")]
    pub fn from_env() -> Result<Self, DshError> {
        if let Ok(path) = utils::get_env_var(VAR_PKI_CONFIG_DIR) {
            Self::from_pki_config_dir(Some(path))
        } else {
            let config_host = utils::get_env_var(VAR_KAFKA_CONFIG_HOST)
                .map(|host| ensure_https_prefix(host))
                .unwrap_or_else(|_| {
                    warn!(
                        "{} is not set, using default value {}",
                        VAR_KAFKA_CONFIG_HOST, DEFAULT_CONFIG_HOST
                    );
                    DEFAULT_CONFIG_HOST.to_string()
                });
            let task_id = utils::get_env_var(VAR_TASK_ID)?;
            let tenant_name = utils::tenant_name()?;
            Self::from_bootstrap(&config_host, &tenant_name, &task_id)
        }
    }

    /// Get the certificates from a directory.
    ///
    /// This method is usefull if you already have the certificates in a directory.
    /// For example if you are using Kafka Proxy, VPN or when a different process already
    /// created the certificates.
    ///
    /// ## Arguments
    /// * `path` - Path to the directory where the certificates are stored (Optional).
    ///
    /// path can be overruled by setting the environment variable `PKI_CONFIG_DIR`.
    ///
    /// ## Note
    /// Only certificates in PEM format are supported.
    /// Key files should be in PKCS8 format and can be DER or PEM files.
    #[cfg(feature = "bootstrap")]
    pub fn from_pki_config_dir<P>(path: Option<P>) -> Result<Self, DshError>
    where
        P: AsRef<std::path::Path>,
    {
        pki_config_dir::get_pki_certificates(path)
    }

    /// Build an async reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_client_config(&self) -> Result<reqwest::ClientBuilder, DshError> {
        let (pem_identity, reqwest_cert) = Self::prepare_reqwest_client(
            self.dsh_kafka_certificate_pem(),
            &self.private_key_pem(),
            self.dsh_ca_certificate_pem(),
        )?;
        Ok(reqwest::Client::builder()
            .add_root_certificate(reqwest_cert)
            .identity(pem_identity)
            .use_rustls_tls())
    }

    /// Build a reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_blocking_client_config(&self) -> Result<ClientBuilder, DshError> {
        let (pem_identity, reqwest_cert) = Self::prepare_reqwest_client(
            self.dsh_kafka_certificate_pem(),
            &self.private_key_pem(),
            self.dsh_ca_certificate_pem(),
        )?;
        Ok(Client::builder()
            .add_root_certificate(reqwest_cert)
            .identity(pem_identity)
            .use_rustls_tls())
    }

    /// Get the root certificate as PEM string. Equivalent to ca.crt.
    pub fn dsh_ca_certificate_pem(&self) -> &str {
        self.dsh_ca_certificate_pem.as_str()
    }

    /// Get the kafka certificate as PEM string. Equivalent to client.pem.
    pub fn dsh_kafka_certificate_pem(&self) -> &str {
        self.dsh_client_certificate_pem.as_str()
    }

    /// Get the private key as PKCS8 and return bytes based on asn1 DER format.
    pub fn private_key_pkcs8(&self) -> Vec<u8> {
        self.key_pair.serialize_der()
    }

    /// Get the private key as PEM string. Equivalent to client.key.
    pub fn private_key_pem(&self) -> String {
        self.key_pair.serialize_pem()
    }

    /// Get the public key as PEM string.
    pub fn public_key_pem(&self) -> String {
        self.key_pair.public_key_pem()
    }

    /// Get the public key as DER bytes.
    pub fn public_key_der(&self) -> Vec<u8> {
        self.key_pair.public_key_der()
    }

    /// Create the ca.crt, client.pem, and client.key files in a desired directory.
    ///
    /// This method will create the directory if it does not exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dsh_sdk::Properties;
    /// use std::path::PathBuf;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let dsh_properties = Properties::get();
    /// let directory = PathBuf::from("dir");
    /// dsh_properties.certificates()?.to_files(&directory)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_files(&self, dir: &PathBuf) -> Result<(), DshError> {
        std::fs::create_dir_all(dir)?;
        Self::create_file(dir.join("ca.crt"), self.dsh_ca_certificate_pem())?;
        Self::create_file(dir.join("client.pem"), self.dsh_kafka_certificate_pem())?;
        Self::create_file(dir.join("client.key"), self.private_key_pem())?;
        Ok(())
    }

    fn create_file<C: AsRef<[u8]>>(path: PathBuf, contents: C) -> Result<(), DshError> {
        std::fs::write(&path, contents)?;
        info!("File created ({})", path.display());
        Ok(())
    }

    fn create_identity(
        cert: &[u8],
        private_key: &[u8],
    ) -> Result<reqwest::Identity, reqwest::Error> {
        let mut ident = private_key.to_vec();
        ident.extend_from_slice(b"\n");
        ident.extend_from_slice(cert);
        reqwest::Identity::from_pem(&ident)
    }

    fn prepare_reqwest_client(
        kafka_certificate: &str,
        private_key: &str,
        ca_certificate: &str,
    ) -> Result<(reqwest::Identity, reqwest::tls::Certificate), DshError> {
        let pem_identity =
            Cert::create_identity(kafka_certificate.as_bytes(), private_key.as_bytes())?;
        let reqwest_cert = reqwest::tls::Certificate::from_pem(ca_certificate.as_bytes())?;
        Ok((pem_identity, reqwest_cert))
    }
}

/// Helper function to ensure that the host starts with `https://` (or `http://`)
fn ensure_https_prefix(host: String) -> String {
    if host.starts_with("https://") || host.starts_with("http://") {
        host
    } else {
        format!("https://{}", host)
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
        let pkey_pem = String::from_utf8_lossy(pkey_pem_bytes.as_slice());
        assert_eq!(key_pem, pkey_pem);
    }

    #[test]
    fn test_public_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey_pub_pem_bytes = pkey.public_key_to_pem().unwrap();

        let pub_pem = cert.public_key_pem();
        let pkey_pub_pem = String::from_utf8_lossy(pkey_pub_pem_bytes.as_slice());
        assert_eq!(pub_pem, pkey_pub_pem);
    }

    #[test]
    fn test_public_key_der() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey_pub_der = pkey.public_key_to_der().unwrap();

        let pub_der = cert.public_key_der();
        assert_eq!(pub_der, pkey_pub_der);
    }

    #[test]
    fn test_private_key_pkcs8() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
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

    #[test]
    fn test_prepare_reqwest_client() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let result = Cert::prepare_reqwest_client(
            cert.dsh_kafka_certificate_pem(),
            &cert.private_key_pem(),
            cert.dsh_ca_certificate_pem(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_reqwest_client_config() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let client = cert.reqwest_client_config();
        assert!(client.is_ok());
    }

    #[test]
    fn test_reqwest_blocking_client_config() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let client = cert.reqwest_blocking_client_config();
        assert!(client.is_ok());
    }

    #[test]
    fn test_ensure_https_prefix() {
        let host = "http://example.com".to_string();
        let result = ensure_https_prefix(host);
        assert_eq!(result, "http://example.com");

        let host = "https://example.com".to_string();
        let result = ensure_https_prefix(host);
        assert_eq!(result, "https://example.com");

        let host = "example.com".to_string();
        let result = ensure_https_prefix(host);
        assert_eq!(result, "https://example.com");
    }
}
