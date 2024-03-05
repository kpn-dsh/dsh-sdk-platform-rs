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
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>>{
//! let dsh_properties = Properties::new().await?;
//! let directory = PathBuf::from("dir");
//! dsh_properties.certificates()?.to_files(&directory)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Reqwest Client
//!
//! With this request client we can retrieve datastreams.json and connect to Schema Registry.

use log::info;
use picky::hash::HashAlgorithm;
use picky::key::PrivateKey;
use picky::signature::SignatureAlgorithm;
use picky::x509::csr::Csr;
use picky::x509::name::{DirectoryName, NameAttr};
use reqwest::{Client, ClientBuilder, Identity};
use std::path::PathBuf;

use super::bootstrap::{Dn, DshCall, DshConfig};

use crate::error::DshError;

/// Hold all relevant certificates and keys to connect to DSH.
///
///
#[derive(Debug, Clone)]
pub struct Cert {
    dsh_ca_certificate_pem: String,
    dsh_kafka_certificate_pem: String,
    private_key: PrivateKey,
}

impl Cert {
    /// Create a new certificate struct.
    pub(crate) async fn new(
        dn: Dn,
        dsh_config: &DshConfig,
        client: &Client,
    ) -> Result<Self, DshError> {
        let private_key = PrivateKey::generate_rsa(4096)?;
        let csr = Self::generate_csr(&private_key, dn).await?;
        let dsh_kafka_certificate_pem = DshCall::CertificateSignRequest {
            config: dsh_config,
            csr: csr.to_pem()?,
        }
        .perform_call(client)
        .await?;
        Ok(Self {
            dsh_ca_certificate_pem: dsh_config.dsh_ca_certificate().to_string(),
            dsh_kafka_certificate_pem,
            private_key,
        })
    }

    /// Build a reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_client_config(&self) -> Result<ClientBuilder, DshError> {
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

    /// Get the private key. This format is from the picky library.
    ///
    /// With this format we can convert it to other formats like PEM.
    /// It allso is able to return the public key.
    pub fn private_key(&self) -> &PrivateKey {
        &self.private_key
    }

    /// Get the private key as PKCS8 and return bytes based on asn1 DER format.
    pub fn private_key_pkcs8(&self) -> Result<Vec<u8>, DshError> {
        Ok(self.private_key().to_pkcs8()?)
    }

    /// Get the private key as PEM string. Equivalent to client.key.
    pub fn private_key_pem(&self) -> Result<String, DshError> {
        Ok(self.private_key().to_pem_str()?)
    }

    /// Get the public key as PEM string.
    pub fn public_key_pem(&self) -> Result<String, DshError> {
        Ok(self.private_key().to_public_key()?.to_pem_str()?)
    }

    /// Get the public key as DER bytes.
    pub fn public_key_der(&self) -> Result<Vec<u8>, DshError> {
        Ok(self.private_key().to_public_key()?.to_der()?)
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
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>>{
    /// let dsh_properties = Properties::new().await?;
    /// let directory = PathBuf::from("dir");
    /// dsh_properties.certificates()?.to_files(&directory)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_files(&self, dir: &PathBuf) -> Result<(), DshError> {
        std::fs::create_dir_all(dir)?;
        Self::create_file(dir.join("ca.crt"), self.dsh_ca_certificate_pem())?;
        Self::create_file(dir.join("client.pem"), self.dsh_kafka_certificate_pem())?;
        Self::create_file(dir.join("client.key"), self.private_key_pkcs8()?)?;
        Ok(())
    }

    /// Generate the certificate signing request.
    ///
    /// Implementation via Picky library.
    async fn generate_csr(
        private_key: &PrivateKey,
        dn: Dn,
    ) -> Result<Csr, picky::x509::csr::CsrError> {
        let mut subject = DirectoryName::new_common_name(dn.cn());
        subject.add_attr(NameAttr::OrganizationalUnitName, dn.ou());
        subject.add_attr(NameAttr::OrganizationName, dn.o());
        Csr::generate(
            subject,
            private_key,
            SignatureAlgorithm::RsaPkcs1v15(HashAlgorithm::SHA2_512),
        )
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

    static TEST_CERTIFICATES: OnceLock<Cert> = OnceLock::new();

    fn set_test_cert() -> Cert {
        Cert{
            dsh_ca_certificate_pem: "-----BEGIN CERTIFICATE-----\nMIIDYDCCAkigAwIBAgIUI--snip--\n-----END CERTIFICATE-----".to_string(),
            dsh_kafka_certificate_pem: "-----BEGIN CERTIFICATE-----\nMIIDYDCCAkigAwIBAgIUI--snip--\n-----END CERTIFICATE-----".to_string(),
            private_key: PrivateKey::generate_rsa(4096).unwrap(),
        }
    }

    #[test]
    fn test_private_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.private_key_pem().unwrap();
        assert!(pem.starts_with("-----BEGIN PRIVATE KEY-----"));
        assert!(pem.ends_with("-----END PRIVATE KEY-----"));
    }

    #[test]
    fn test_public_key_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.public_key_pem().unwrap();
        assert!(pem.starts_with("-----BEGIN PUBLIC KEY-----"));
        assert!(pem.ends_with("-----END PUBLIC KEY-----"));
    }

    #[test]
    fn test_private_key_pkcs8() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pkcs8 = cert.private_key_pkcs8().unwrap();
        let picky_pks8 = PrivateKey::from_pkcs8(&pkcs8).unwrap();
        assert_eq!(cert.private_key(), &picky_pks8);
    }

    #[test]
    fn test_public_key_der() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let der = cert.public_key_der().unwrap();
        let picky_der = cert
            .private_key()
            .to_public_key()
            .unwrap()
            .to_der()
            .unwrap();
        assert_eq!(der, picky_der);
    }

    #[test]
    fn test_dsh_ca_certificate_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.dsh_ca_certificate_pem();
        assert!(pem.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(pem.ends_with("-----END CERTIFICATE-----"));
    }

    #[test]
    fn test_dsh_kafka_certificate_pem() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let pem = cert.dsh_kafka_certificate_pem();
        assert!(pem.starts_with("-----BEGIN CERTIFICATE-----"));
        assert!(pem.ends_with("-----END CERTIFICATE-----"));
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

    #[tokio::test]
    async fn test_dsh_certificate_sign_request() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let dn = Dn::parse_string("CN=Test CN,OU=Test OU,O=Test Org").unwrap();
        let csr = Cert::generate_csr(&cert.private_key(), dn).await.unwrap();
        let (directory_name, pub_key) = csr.into_subject_infos();
        assert_eq!(
            directory_name.to_string(),
            "CN=Test CN,OU=Test OU,O=Test Org"
        );
        assert_eq!(
            pub_key.to_pem_str().unwrap(),
            cert.public_key_pem().unwrap()
        );
    }
}
