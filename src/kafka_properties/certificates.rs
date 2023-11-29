use picky::hash::HashAlgorithm;
use picky::key::PrivateKey;
use picky::signature::SignatureAlgorithm;
use picky::x509::csr::Csr;
use picky::x509::name::{DirectoryName, NameAttr};

use reqwest::{Client, ClientBuilder, Identity};

use super::dsh::{Dn, DshCall, DshConfig};

use crate::error::DshError;

/// Hold all relevant certificates and keys to connect to DSH.
#[derive(Debug, Clone)]
pub struct Cert {
    dsh_ca_certificate: String,
    dsh_kafka_certificate: String,
    private_key: String,
    public_key: String,
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
        let dsh_kafka_certificate = DshCall::CertificateSignRequest {
            config: &dsh_config,
            csr: csr.to_pem()?,
        }
        .perform_call(&client)
        .await?;
        Ok(Self {
            dsh_ca_certificate: dsh_config.dsh_ca_certificate().to_string(),
            dsh_kafka_certificate,
            private_key: private_key.to_pem_str()?,
            public_key: private_key.to_public_key()?.to_pem_str()?,
        })
    }

    /// Build a reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_client_config(&self) -> Result<ClientBuilder, DshError> {
        let pem_identity = Cert::create_identity(
            self.dsh_kafka_certificate_pem().as_bytes(),
            self.private_key_pem().as_bytes(),
        )?;
        let reqwest_cert =
            reqwest::tls::Certificate::from_pem(self.dsh_ca_certificate_pem().as_bytes())?;
        Ok(Client::builder()
            .add_root_certificate(reqwest_cert)
            .identity(pem_identity)
            .use_rustls_tls())
    }

    /// Get the root certificate as PEM string. Equivalent to ca.crt.
    pub fn dsh_ca_certificate_pem(&self) -> String {
        self.dsh_ca_certificate.clone()
    }

    /// Get the kafka certificate as PEM string. Equivalent to client.pem.
    pub fn dsh_kafka_certificate_pem(&self) -> String {
        self.dsh_kafka_certificate.clone()
    }

    /// Get the private key as PEM string. Equivalent to client.key.
    pub fn private_key_pem(&self) -> String {
        self.private_key.clone()
    }

    /// Get the public key as PEM string.
    pub fn public_key_pem(&self) -> String {
        self.public_key.clone()
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

    #[tokio::test]
    #[ignore] // This is ignored because it takes a long time to generate the private key in debug mode.
    async fn test_dsh_certificate_sign_request() {
        let private_key = PrivateKey::generate_rsa(4096).unwrap();
        let dn = Dn::parse_string("CN=Test CN,OU=Test OU,O=Test Org").unwrap();
        let csr = Cert::generate_csr(&private_key, dn).await.unwrap();
        let (directory_name, pub_key) = csr.into_subject_infos();
        assert_eq!(
            directory_name.to_string(),
            "CN=Test CN,OU=Test OU,O=Test Org"
        );
        assert_eq!(
            pub_key.to_pem_str().unwrap(),
            private_key.to_public_key().unwrap().to_pem_str().unwrap()
        );
    }
}
