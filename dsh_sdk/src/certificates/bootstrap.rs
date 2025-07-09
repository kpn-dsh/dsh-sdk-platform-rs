//! Module for bootstrapping to DSH.
//!
//! This module contains the logic to connect to DSH and retrieve the certificates and datastreams.json
//! to create the properties struct. It follows the certificate signing request pattern as normally
//! used in the get_signed_certificates_json.sh script.
//!
//! ## Note
//! This module is NOT intended to be used directly, but through [Cert] or indirectly via [Properties](crate::Properties).
use log::{debug, info};
use reqwest::blocking::Client;

use rcgen::string::Ia5String;
use rcgen::{CertificateParams, CertificateSigningRequest, DnType, KeyPair, SanType};

use super::CertificatesError;

use super::Cert;
use crate::utils;
use crate::{
    VAR_DSH_CA_CERTIFICATE, VAR_DSH_CONTAINER_DNS_NAME, VAR_DSH_SECRET_TOKEN,
    VAR_DSH_SECRET_TOKEN_PATH,
};

/// Connect to DSH and retrieve the certificates and datastreams.json to create the properties struct
pub(crate) fn bootstrap(
    config_host: &str,
    tenant_name: &str,
    task_id: &str,
) -> Result<Cert, CertificatesError> {
    let dsh_config = DshBootstrapConfig::new(config_host, tenant_name, task_id)?;
    let client = reqwest_ca_client(dsh_config.dsh_ca_certificate.as_bytes())?;
    let dn = DshBootstapCall::Dn(&dsh_config).retryable_call(&client)?;
    let dn = Dn::parse_string(&dn)?;
    let certificates = get_signed_client_cert(dn, &dsh_config, &client)?;
    info!("Successfully connected to DSH");
    Ok(certificates)
}

/// Build a request client with the DSH CA certificate.
fn reqwest_ca_client(dsh_ca_certificate: &[u8]) -> Result<Client, reqwest::Error> {
    let reqwest_cert = reqwest::tls::Certificate::from_pem(dsh_ca_certificate)?;
    let client = Client::builder()
        .add_root_certificate(reqwest_cert)
        .build()?;
    Ok(client)
}

/// Generate private key and call for a signed certificate to DSH.
fn get_signed_client_cert(
    dn: Dn,
    dsh_config: &DshBootstrapConfig,
    client: &Client,
) -> Result<Cert, CertificatesError> {
    let key_pair = KeyPair::generate_for(&rcgen::PKCS_ECDSA_P384_SHA384)?;
    let csr = generate_csr(&key_pair, dn)?;
    let client_certificate = DshBootstapCall::CertificateSignRequest {
        config: dsh_config,
        csr: &csr.pem()?,
    }
    .retryable_call(client)?;
    let ca_cert = pem::parse_many(&dsh_config.dsh_ca_certificate)?;
    let client_cert = pem::parse_many(client_certificate)?;
    Ok(Cert::new(
        pem::encode_many(&ca_cert),
        pem::encode_many(&client_cert),
        key_pair,
    ))
}

/// Generate the certificate signing request.
fn generate_csr(
    key_pair: &KeyPair,
    dn: Dn,
) -> Result<CertificateSigningRequest, CertificatesError> {
    let mut params = CertificateParams::default();
    params.distinguished_name.push(DnType::CommonName, dn.cn);
    params
        .distinguished_name
        .push(DnType::OrganizationalUnitName, dn.ou);
    params
        .distinguished_name
        .push(DnType::OrganizationName, dn.o);
    if let Some(ia5_string) = utils::get_env_var(VAR_DSH_CONTAINER_DNS_NAME)
        .ok()
        .and_then(|dns_string| Ia5String::try_from(dns_string).ok())
    {
        params.subject_alt_names.push(SanType::DnsName(ia5_string));
    }
    Ok(params.serialize_request(key_pair)?)
}

/// Helper struct to store the config needed for bootstrapping to DSH
#[derive(Debug)]
struct DshBootstrapConfig<'a> {
    config_host: &'a str,
    tenant_name: &'a str,
    task_id: &'a str,
    dsh_secret_token: String,
    dsh_ca_certificate: String,
}
impl<'a> DshBootstrapConfig<'a> {
    fn new(
        config_host: &'a str,
        tenant_name: &'a str,
        task_id: &'a str,
    ) -> Result<Self, CertificatesError> {
        let dsh_secret_token = match utils::get_env_var(VAR_DSH_SECRET_TOKEN) {
            Ok(token) => token,
            Err(_) => {
                // if DSH_SECRET_TOKEN is not set, try to read it from a file (for system space applications)
                debug!("trying to read DSH_SECRET_TOKEN from file");
                let secret_token_path = utils::get_env_var(VAR_DSH_SECRET_TOKEN_PATH)?;
                let path = std::path::PathBuf::from(secret_token_path);
                std::fs::read_to_string(path)?
            }
        };
        let dsh_ca_certificate = utils::get_env_var(VAR_DSH_CA_CERTIFICATE)?;
        Ok(DshBootstrapConfig {
            config_host,
            task_id,
            tenant_name,
            dsh_secret_token,
            dsh_ca_certificate,
        })
    }
}

enum DshBootstapCall<'a> {
    /// Call to retreive distinguished name.
    Dn(&'a DshBootstrapConfig<'a>),
    /// Call to post the certificate signing request.
    CertificateSignRequest {
        config: &'a DshBootstrapConfig<'a>,
        csr: &'a str,
    },
}

impl DshBootstapCall<'_> {
    fn url(&self) -> String {
        match self {
            DshBootstapCall::Dn(config) => {
                format!(
                    "{}/dn/{}/{}",
                    config.config_host, config.tenant_name, config.task_id
                )
            }
            DshBootstapCall::CertificateSignRequest { config, .. } => {
                format!(
                    "{}/sign/{}/{}",
                    config.config_host, config.tenant_name, config.task_id
                )
            }
        }
    }

    fn request_builder(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        let url = self.url();
        match self {
            DshBootstapCall::Dn(..) => client.get(url),
            DshBootstapCall::CertificateSignRequest { config, csr, .. } => client
                .post(url)
                .header("X-Kafka-Config-Token", &config.dsh_secret_token)
                .body(csr.to_string()),
        }
    }

    fn perform_call(&self, client: &Client) -> Result<String, CertificatesError> {
        let response = self.request_builder(client).send()?;
        if !response.status().is_success() {
            return Err(CertificatesError::DshCallError {
                url: self.url(),
                status_code: response.status(),
                error_body: response.text().unwrap_or_default(),
            });
        }
        Ok(response.text()?)
    }

    pub(crate) fn retryable_call(&self, client: &Client) -> Result<String, CertificatesError> {
        let mut retries = 0;
        loop {
            match self.perform_call(client) {
                Ok(response) => return Ok(response),
                Err(err) => {
                    if retries >= 30 {
                        return Err(err);
                    }
                    retries += 1;
                    // sleep exponentially
                    let sleep: u64 = std::cmp::min(2u64.pow(retries), 60);
                    log::warn!(
                        "Retrying call to DSH in {sleep} seconds due to error: {}",
                        crate::error::report(&err)
                    );
                    std::thread::sleep(std::time::Duration::from_secs(sleep));
                }
            }
        }
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
    fn parse_string(dn_string: &str) -> Result<Self, CertificatesError> {
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
            cn: cn.ok_or(CertificatesError::ParseDn(
                "CN is missing in DN string".to_string(),
            ))?,
            ou: ou.ok_or(CertificatesError::ParseDn(
                "OU is missing in DN string".to_string(),
            ))?,
            o: o.ok_or(CertificatesError::ParseDn(
                "O is missing in DN string".to_string(),
            ))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::str::from_utf8;

    use rcgen::{CertifiedKey, generate_simple_self_signed};
    use std::sync::OnceLock;

    use openssl::pkey::PKey;
    use openssl::x509::X509Req;

    static TEST_CERTIFICATES: OnceLock<Cert> = OnceLock::new();

    fn set_test_cert() -> Cert {
        let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];
        let CertifiedKey { cert, signing_key } =
            generate_simple_self_signed(subject_alt_names).unwrap();
        Cert::new(cert.pem(), cert.pem(), signing_key)
    }

    #[test]
    fn test_dsh_call_request_builder() {
        let dsh_config = DshBootstrapConfig {
            config_host: "https://test_host",
            tenant_name: "test_tenant_name",
            task_id: "test_task_id",
            dsh_secret_token: "test_token".to_string(),
            dsh_ca_certificate: "test_ca_certificate".to_string(),
        };
        let builder: reqwest::blocking::RequestBuilder =
            DshBootstapCall::Dn(&dsh_config).request_builder(&Client::new());
        let request = builder.build().unwrap();
        assert_eq!(request.method().as_str(), "GET");
        let csr = "-----BEGIN test_type-----\n-----END test_type-----";
        let builder: reqwest::blocking::RequestBuilder = DshBootstapCall::CertificateSignRequest {
            config: &dsh_config,
            csr,
        }
        .request_builder(&Client::new());
        let request = builder.build().unwrap();
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
        assert_eq!(body, csr);
    }

    #[test]
    fn test_dsh_call_perform() {
        // Create a mock for the expected HTTP request
        let mut dsh = mockito::Server::new();
        let dn = "CN=test_cn,OU=test_ou,O=test_o";
        dsh.mock("GET", "/dn/tenant/test_task_id")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(dn)
            .create();
        // simple reqwest client
        let client = Client::new();
        // create a DshBootstrapConfig struct
        let dsh_config = DshBootstrapConfig {
            config_host: &dsh.url(),
            tenant_name: "tenant",
            task_id: "test_task_id",
            dsh_secret_token: "test_token".to_string(),
            dsh_ca_certificate: "test_ca_certificate".to_string(),
        };
        // call the function
        let response = DshBootstapCall::Dn(&dsh_config)
            .perform_call(&client)
            .unwrap();
        assert_eq!(response, dn);
    }

    #[test]
    fn test_dsh_parse_dn() {
        let dn_string = "CN=test_cn,OU=test_ou,O=test_o";
        let dn = Dn::parse_string(dn_string).unwrap();
        assert_eq!(dn.cn, "test_cn");
        assert_eq!(dn.ou, "test_ou");
        assert_eq!(dn.o, "test_o");
    }

    #[test]
    fn test_dsh_certificate_sign_request() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let dn = Dn::parse_string("CN=Test CN,OU=Test OU,O=Test Org").unwrap();
        let csr = generate_csr(&cert.key_pair, dn).unwrap();
        let req = csr.pem().unwrap();
        assert!(req.starts_with("-----BEGIN CERTIFICATE REQUEST-----"));
        assert!(req.trim().ends_with("-----END CERTIFICATE REQUEST-----"));
    }

    #[test]
    fn test_verify_csr() {
        let cert = TEST_CERTIFICATES.get_or_init(set_test_cert);
        let dn = Dn::parse_string("CN=Test CN,OU=Test OU,O=Test Org").unwrap();
        let csr = generate_csr(&cert.key_pair, dn).unwrap();
        let csr_pem = csr.pem().unwrap();
        let key = cert.private_key_pkcs8();
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

    #[test]
    #[serial(env_dependency)]
    fn test_dsh_config_new() {
        unsafe {
            // normal situation where DSH variables are set
            env::set_var(VAR_DSH_SECRET_TOKEN, "test_token");
            env::set_var(VAR_DSH_CA_CERTIFICATE, "test_ca_certificate");
            let config_host = "https://test_host";
            let tenant_name = "test_tenant";
            let task_id = "test_task_id";
            let dsh_config = DshBootstrapConfig::new(config_host, tenant_name, task_id).unwrap();
            assert_eq!(dsh_config.config_host, "https://test_host");
            assert_eq!(dsh_config.task_id, "test_task_id");
            assert_eq!(dsh_config.tenant_name, "test_tenant");
            assert_eq!(dsh_config.dsh_secret_token, "test_token");
            assert_eq!(dsh_config.dsh_ca_certificate, "test_ca_certificate");
            // DSH_SECRET_TOKEN is not set, but DSH_SECRET_TOKEN_PATH is set
            env::remove_var(VAR_DSH_SECRET_TOKEN);
            let test_token_dir = "test_files";
            std::fs::create_dir_all(test_token_dir).unwrap();
            let test_token_dir = format!("{}/test_token", test_token_dir);
            let _ = std::fs::remove_file(&test_token_dir);
            env::set_var(VAR_DSH_SECRET_TOKEN_PATH, &test_token_dir);
            let result = DshBootstrapConfig::new(config_host, tenant_name, task_id);
            assert!(result.is_err());
            std::fs::write(test_token_dir.as_str(), "test_token_from_file").unwrap();
            let dsh_config = DshBootstrapConfig::new(config_host, tenant_name, task_id).unwrap();
            assert_eq!(dsh_config.dsh_secret_token, "test_token_from_file");
            // fail if DSH_CA_CERTIFICATE is not set
            env::remove_var(VAR_DSH_CA_CERTIFICATE);
            let result = DshBootstrapConfig::new(config_host, tenant_name, task_id);
            assert!(result.is_err());
            env::remove_var(VAR_DSH_SECRET_TOKEN);
            env::remove_var(VAR_DSH_CA_CERTIFICATE);
            env::remove_var(VAR_DSH_SECRET_TOKEN_PATH);
        }
    }
}
