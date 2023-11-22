use log::warn;
use reqwest::{Client, ClientBuilder, Identity};

use picky::key::PrivateKey;
use picky::x509::csr::Csr;
use picky::x509::name::{DirectoryName, NameAttr};
use picky::{hash::HashAlgorithm, signature::SignatureAlgorithm};

use std::env;

use crate::error::DshError;

use super::{Bootstrap, KafkaProperties};

impl Bootstrap {
    /// Create a new bootstrap struct to connect to DSH
    /// This function will call the DSH API to retrieve the certificates and datastreams.json
    pub(crate) async fn new_dsh() -> Result<Self, DshError> {
        let dsh_config = DshConfig::new()?;
        let client = Bootstrap::reqwest_client(dsh_config.dsh_ca_certificate.as_bytes())?;
        let dn = DshCall::Dn(&dsh_config).perform_call(&client).await?;
        let dn = Dn::parse_string(&dn)?;
        let private_key = PrivateKey::generate_rsa(4096)?;
        let csr = Cert::generate_csr(&private_key, dn).await?;
        let dsh_kafka_certificate = DshCall::CertificateSignRequest {
            config: &dsh_config,
            csr: csr.to_pem()?,
        }
        .perform_call(&client)
        .await?;
        let certificates = Cert {
            dsh_ca_certificate: dsh_config.dsh_ca_certificate.to_string(),
            dsh_kafka_certificate,
            private_key: private_key.to_pem_str()?,
            public_key: private_key.to_public_key()?.to_pem_str()?,
        };
        let client_with_cert = certificates.reqwest_client_config()?.build()?;
        let datastreams_string = DshCall::Datastream(&dsh_config)
            .perform_call(&client_with_cert)
            .await?;
        let kafka_properties: KafkaProperties = serde_json::from_str(&datastreams_string)?;
        Ok(Bootstrap {
            kafka_properties,
            client_id: dsh_config.task_id.to_string(),
            certificates: Some(certificates),
        })
    }

    /// Build a request client with the DSH CA certificate.
    fn reqwest_client(dsh_ca_certificate: &[u8]) -> Result<Client, reqwest::Error> {
        let reqwest_cert = reqwest::tls::Certificate::from_pem(dsh_ca_certificate)?;
        let client = Client::builder()
            .add_root_certificate(reqwest_cert)
            .build()?;
        Ok(client)
    }
}

/// Hold all relevant certificates and keys to connect to DSH.
#[derive(Debug, Clone)]
pub struct Cert {
    dsh_ca_certificate: String,
    dsh_kafka_certificate: String,
    private_key: String,
    public_key: String,
}

impl Cert {
    /// Build a reqwest client with the DSH Kafka certificate included.
    /// With this client we can retrieve datastreams.json and conenct to Schema Registry.
    pub fn reqwest_client_config(&self) -> Result<ClientBuilder, reqwest::Error> {
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
        let mut subject = DirectoryName::new_common_name(dn.cn);
        subject.add_attr(NameAttr::OrganizationalUnitName, dn.ou);
        subject.add_attr(NameAttr::OrganizationName, dn.o);
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

struct DshConfig {
    config_host: String,
    tenant_name: String,
    task_id: String,
    dsh_secret_token: String,
    dsh_ca_certificate: String,
}

/// Helper struct to store the config needed for bootstrapping to DSH
impl DshConfig {
    fn new() -> Result<Self, env::VarError> {
        let config_host = format!("{}{}", "https://", env::var("KAFKA_CONFIG_HOST")?);
        let task_id = env::var("MESOS_TASK_ID")?;
        let app_id = env::var("MARATHON_APP_ID")?;
        let dsh_secret_token = env::var("DSH_SECRET_TOKEN")?;
        let dsh_ca_certificate = env::var("DSH_CA_CERTIFICATE")?;
        let tenant_name = DshConfig::tenant_name(app_id);
        Ok(DshConfig {
            config_host,
            task_id,
            tenant_name,
            dsh_secret_token,
            dsh_ca_certificate,
        })
    }

    /// Derive the tenant name from the app id.
    fn tenant_name(app_id: String) -> String {
        let tenant_name = app_id.split('/').nth(1);
        match tenant_name {
            Some(tenant_name) => tenant_name.to_string(),
            None => {
                warn!(
                    "MARATHON_APP_ID is not as expected, missing expected slashes, using \"{}\" as tenant name",
                    app_id
                );
                app_id
            }
        }
    }
}

enum DshCall<'a> {
    /// Call to retreive distinguished name.
    Dn(&'a DshConfig),
    /// Call to retreive datastreams.json.
    Datastream(&'a DshConfig),
    /// Call to post the certificate signing request.
    CertificateSignRequest {
        config: &'a DshConfig,
        csr: picky::pem::Pem<'a>,
    },
}

impl DshCall<'_> {
    fn url_for_call(&self) -> String {
        match self {
            DshCall::Dn(config) => {
                format!(
                    "{}/dn/{}/{}",
                    config.config_host, config.tenant_name, config.task_id
                )
            }
            DshCall::Datastream(config) => {
                format!(
                    "{}/kafka/config/{}/{}",
                    config.config_host, config.tenant_name, config.task_id
                )
            }
            DshCall::CertificateSignRequest { config, .. } => {
                format!(
                    "{}/sign/{}/{}",
                    config.config_host, config.tenant_name, config.task_id
                )
            }
        }
    }

    fn request_builder(&self, url: &str, client: &Client) -> reqwest::RequestBuilder {
        match self {
            DshCall::Dn(..) | DshCall::Datastream(..) => client.get(url),
            DshCall::CertificateSignRequest { config, csr, .. } => client
                .post(url)
                .header("X-Kafka-Config-Token", &config.dsh_secret_token)
                .body(csr.to_string()),
        }
    }

    async fn perform_call(&self, client: &Client) -> Result<String, DshError> {
        let url = self.url_for_call();
        let response = self.request_builder(&url, client).send().await?;
        if !response.status().is_success() {
            return Err(DshError::DshCallError {
                url,
                status_code: response.status(),
                error_body: response.text().await?,
            });
        }
        Ok(response.text().await?)
    }
}

/// Struct to parse DN string into separate fields.
/// Needed for Picky solution.
#[derive(Debug)]
struct Dn {
    cn: String,
    ou: String,
    o: String,
}

impl Dn {
    /// Parse the DN string into Dn struct.
    fn parse_string(dn_string: &str) -> Result<Self, DshError> {
        let mut cn = None;
        let mut ou = None;
        let mut o = None;

        for segment in dn_string.split(',') {
            let parts: Vec<&str> = segment.split('=').collect();
            if parts.len() == 2 {
                match parts[0] {
                    "CN" => cn = Some(parts[1].to_string()),
                    "OU" => ou = Some(parts[1].to_string()),
                    "O" => o = Some(parts[1].to_string()),
                    _ => (),
                }
            }
        }

        Ok(Dn {
            cn: cn.ok_or(DshError::ParseDnError(
                "CN is missing in DN string".to_string(),
            ))?,
            ou: ou.ok_or(DshError::ParseDnError(
                "OU is missing in DN string".to_string(),
            ))?,
            o: o.ok_or(DshError::ParseDnError(
                "O is missing in DN string".to_string(),
            ))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn test_dsh_config_tenant_name() {
        let app_id = "/greenbox-dev/app-name".to_string();
        let result = DshConfig::tenant_name(app_id.clone());
        assert_eq!(
            result,
            "greenbox-dev".to_string(),
            "{} is not parsed correctly",
            app_id
        );
        let app_id = "greenbox-dev".to_string();
        let result = DshConfig::tenant_name(app_id.clone());
        assert_eq!(
            result, app_id,
            "{} is not parsed correctly, should be the same",
            app_id
        );
    }

    #[test]
    fn test_dsh_call_request_builder() {
        let dsh_config = DshConfig {
            config_host: "https://test_host".to_string(),
            tenant_name: "test_tenant_name".to_string(),
            task_id: "test_task_id".to_string(),
            dsh_secret_token: "test_token".to_string(),
            dsh_ca_certificate: "test_ca_certificate".to_string(),
        };
        let builder: reqwest::RequestBuilder =
            DshCall::Dn(&dsh_config).request_builder("https://test_host", &reqwest::Client::new());
        let (_, request) = builder.build_split();
        let request = request.unwrap();
        assert_eq!(request.method().as_str(), "GET");
        let builder: reqwest::RequestBuilder = DshCall::Datastream(&dsh_config)
            .request_builder("https://test_host", &reqwest::Client::new());
        let (_, request) = builder.build_split();
        let request = request.unwrap();
        assert_eq!(request.method().as_str(), "GET");
        let pem = picky::pem::Pem::new("test_type", "test".as_bytes());
        let builder: reqwest::RequestBuilder = DshCall::CertificateSignRequest {
            config: &dsh_config,
            csr: pem,
        }
        .request_builder("https://test_host", &reqwest::Client::new());
        let (_, request) = builder.build_split();
        let request = request.unwrap();
        assert_eq!(request.method().as_str(), "POST");
        assert_eq!(
            request
                .headers()
                .get("X-Kafka-Config-Token")
                .unwrap()
                .to_str()
                .unwrap(),
            "test_token"
        );
        let body = from_utf8(request.body().unwrap().as_bytes().unwrap()).unwrap();
        assert!(body.contains("-----BEGIN test_type-----"));
    }

    #[tokio::test]
    async fn test_dsh_call_perform() {
        // Create a mock for the expected HTTP request
        let mut dsh = mockito::Server::new();
        let dn = "CN=test_cn,OU=test_ou,O=test_o";
        dsh.mock("GET", "/dn/tenant/test_task_id")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(dn)
            .create();
        // simple reqwest client
        let client = reqwest::Client::new();
        // create a DshConfig struct
        let dsh_config = DshConfig {
            config_host: dsh.url(),
            tenant_name: "tenant".to_string(),
            task_id: "test_task_id".to_string(),
            dsh_secret_token: "test_token".to_string(),
            dsh_ca_certificate: "test_ca_certificate".to_string(),
        };
        // call the function
        let response = DshCall::Dn(&dsh_config)
            .perform_call(&client)
            .await
            .unwrap();
        assert_eq!(response, dn);
    }

    #[tokio::test]
    #[ignore] // This is ignored because it takes a long time to generate the private key in debug mode.
    async fn test_dsh_certificate_sign_request() {
        let private_key = PrivateKey::generate_rsa(4096).unwrap();
        let dn = Dn {
            cn: "Test CN".to_string(),
            ou: "Test OU".to_string(),
            o: "Test Org".to_string(),
        };
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

    #[test]
    fn test_dsh_parse_dn() {
        let dn_string = "CN=test_cn,OU=test_ou,O=test_o";
        let dn = Dn::parse_string(dn_string).unwrap();
        assert_eq!(dn.cn, "test_cn");
        assert_eq!(dn.ou, "test_ou");
        assert_eq!(dn.o, "test_o");
    }
}
