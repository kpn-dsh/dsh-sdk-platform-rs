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
//! use dsh_sdk::dsh::Properties;
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let dsh_properties = Properties::get();
//! let directory = PathBuf::from("dir");
//! dsh_properties.certificates()?.to_files(&directory)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Reqwest Client
//!
//! With this request client we can retrieve datastreams.json and connect to Schema Registry.

use std::sync::Arc;

use log::info;
//use picky::hash::HashAlgorithm;
//use picky::key::PrivateKey;
//use picky::signature::SignatureAlgorithm;
//use picky::x509::csr::Csr;
//use picky::x509::name::{DirectoryName, NameAttr};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Identity;
use std::path::PathBuf;

use super::bootstrap::{Dn, DshCall, DshConfig};

use crate::error::DshError;

use rcgen::{CertificateParams, CertificateSigningRequest, DnType, KeyPair};

/// Hold all relevant certificates and keys to connect to DSH.
///
///
#[derive(Debug, Clone)]
pub struct Cert {
    dsh_ca_certificate_pem: String,
    dsh_kafka_certificate_pem: String,
    key_pair: Arc<KeyPair>,
}

impl Cert {
    /// Create a new certificate struct.
    pub(crate) fn new(dn: Dn, dsh_config: &DshConfig, client: &Client) -> Result<Self, DshError> {
        let key_pair = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P384_SHA384)?;
        let csr = Self::generate_csr(&key_pair, dn)?;
        let dsh_kafka_certificate_pem = DshCall::CertificateSignRequest {
            config: dsh_config,
            csr: &csr.pem()?,
        }
        .perform_call(client)?;
        Ok(Self {
            dsh_ca_certificate_pem: dsh_config.dsh_ca_certificate().to_string(),
            dsh_kafka_certificate_pem,
            key_pair: Arc::new(key_pair),
        })
    }

    /// Build an async reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_client_config(&self) -> Result<reqwest::ClientBuilder, DshError> {
        let pem_identity = Cert::create_identity(
            self.dsh_kafka_certificate_pem().as_bytes(),
            self.private_key_pem()?.as_bytes(),
        )?;
        let reqwest_cert =
            reqwest::tls::Certificate::from_pem(self.dsh_ca_certificate_pem().as_bytes())?;
        Ok(reqwest::Client::builder()
            .add_root_certificate(reqwest_cert)
            .identity(pem_identity)
            .use_rustls_tls())
    }

    /// Build a reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_blocking_client_config(&self) -> Result<ClientBuilder, DshError> {
        let pem_identity = Cert::create_identity(
            self.dsh_kafka_certificate_pem().as_bytes(),
            self.private_key_pem()?.as_bytes(),
        )?;
        let reqwest_cert =
            reqwest::tls::Certificate::from_pem(self.dsh_ca_certificate_pem().as_bytes())?;
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
        self.dsh_kafka_certificate_pem.as_str()
    }

    /// Get the private key as PKCS8 and return bytes based on asn1 DER format.
    pub fn private_key_pkcs8(&self) -> Result<Vec<u8>, DshError> {
        Ok(self.key_pair.serialize_der())
    }

    /// Get the private key as PEM string. Equivalent to client.key.
    pub fn private_key_pem(&self) -> Result<String, DshError> {
        Ok(self.key_pair.serialize_pem())
    }

    /// Get the public key as PEM string.
    pub fn public_key_pem(&self) -> Result<String, DshError> {
        Ok(self.key_pair.public_key_pem())
    }

    /// Get the public key as DER bytes.
    pub fn public_key_der(&self) -> Result<Vec<u8>, DshError> {
        Ok(self.key_pair.public_key_der())
    }

    /// Create the ca.crt, client.pem, and client.key files in a desired directory.
    ///
    /// This method will create the directory if it does not exist.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use dsh_sdk::dsh::Properties;
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
        Self::create_file(dir.join("client.key"), self.private_key_pem()?)?;
        Ok(())
    }

    /// Generate the certificate signing request.
    fn generate_csr(key_pair: &KeyPair, dn: Dn) -> Result<CertificateSigningRequest, DshError> {
        let mut params = CertificateParams::default();
        params.distinguished_name.push(DnType::CommonName, dn.cn());
        params
            .distinguished_name
            .push(DnType::OrganizationalUnitName, dn.ou());
        params
            .distinguished_name
            .push(DnType::OrganizationName, dn.o());
        Ok(params.serialize_request(key_pair)?)
    }

    fn create_file<C: AsRef<[u8]>>(path: PathBuf, contents: C) -> Result<(), DshError> {
        std::fs::write(&path, contents)?;
        info!("File created ({})", path.display());
        Ok(())
    }

    fn create_identity(cert: &[u8], private_key: &[u8]) -> Result<Identity, reqwest::Error> {
        let mut ident = private_key.to_vec();
        ident.extend_from_slice(b"\n");
        ident.extend_from_slice(cert);
        Identity::from_pem(&ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    use openssl::pkey::PKey;
    use openssl::x509::X509Req;

    static TEST_CERTIFICATES: OnceLock<Cert> = OnceLock::new();
    static CA_CERT: &str =
        "-----BEGIN CERTIFICATE-----\nMIIDYDCCAkigAwIBAgIUI--snip--\n-----END CERTIFICATE-----";
    static KAFKA_CERT: &str =
        "-----BEGIN CERTIFICATE-----\nMIIDYDCCAkigAwIBAgIUI--snip--\n-----END CERTIFICATE-----";

    fn set_test_cert() -> Cert {
        Cert {
            dsh_ca_certificate_pem: CA_CERT.to_string(),
            dsh_kafka_certificate_pem: KAFKA_CERT.to_string(),
            key_pair: Arc::new(KeyPair::generate().unwrap()),
        }
    }

    #[test]
    fn test_private_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey_pem_bytes = pkey.private_key_to_pem_pkcs8().unwrap();

        let key_pem = cert.private_key_pem().unwrap();
        let pkey_pem = String::from_utf8_lossy(pkey_pem_bytes.as_slice());
        assert_eq!(key_pem, pkey_pem);
    }

    #[test]
    fn test_public_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey_pub_pem_bytes = pkey.public_key_to_pem().unwrap();

        let pub_pem = cert.public_key_pem().unwrap();
        let pkey_pub_pem = String::from_utf8_lossy(pkey_pub_pem_bytes.as_slice());
        assert_eq!(pub_pem, pkey_pub_pem);
    }

    #[test]
    fn test_public_key_der() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey_pub_der = pkey.public_key_to_der().unwrap();

        let pub_der = cert.public_key_der().unwrap();
        assert_eq!(pub_der, pkey_pub_der);
    }

    #[test]
    fn test_private_key_pkcs8() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.key_pair.serialize_der();
        let pkey = PKey::private_key_from_der(der.as_slice()).unwrap();
        let pkey = pkey.private_key_to_pkcs8().unwrap();

        let key = cert.private_key_pkcs8().unwrap();
        assert_eq!(key, pkey);
    }

    #[test]
    fn test_dsh_ca_certificate_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.dsh_ca_certificate_pem();
        assert_eq!(pem, CA_CERT);
    }

    #[test]
    fn test_dsh_kafka_certificate_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.dsh_kafka_certificate_pem();
        assert_eq!(pem, KAFKA_CERT);
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
    fn test_dsh_certificate_sign_request() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let dn = Dn::parse_string("CN=Test CN,OU=Test OU,O=Test Org").unwrap();
        let csr = Cert::generate_csr(&cert.key_pair, dn).unwrap();
        let req = csr.pem().unwrap();
        assert!(req.starts_with("-----BEGIN CERTIFICATE REQUEST-----"));
        assert!(req.trim().ends_with("-----END CERTIFICATE REQUEST-----"));
    }

    #[test]
    fn test_verify_csr() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let dn = Dn::parse_string("CN=Test CN,OU=Test OU,O=Test Org").unwrap();
        let csr = Cert::generate_csr(&cert.key_pair, dn).unwrap();
        let csr_pem = csr.pem().unwrap();
        let key = cert.private_key_pkcs8().unwrap();
        let pkey = PKey::private_key_from_der(&key).unwrap();

        let req = X509Req::from_pem(csr_pem.as_bytes()).unwrap();
        req.verify(&pkey).unwrap();
        let subject = req
            .subject_name()
            .entries()
            .into_iter()
            .map(|e| e.data().as_utf8().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(",");
        assert_eq!(subject, "Test CN,Test OU,Test Org");
    }
}
