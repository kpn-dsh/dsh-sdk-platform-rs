//! Module for bootstrapping the DSH client.
//!
//! This module contains the logic to connect to DSH and retrieve the certificates and datastreams.json
//! to create the properties struct. It follows the certificate signing request pattern as normally
//! used in the get_signed_certificates_json.sh script.
//!
//! ## Note
//! This module is not intended to be used directly, but through the `Properties` struct. It will
//! always be used when getting a `Properties` struct via dsh::Properties::get().
use log::{debug, info};
use reqwest::blocking::Client;

use crate::error::DshError;

use super::certificates::Cert;
use crate::utils;
use crate::{VAR_DSH_CA_CERTIFICATE, VAR_DSH_SECRET_TOKEN, VAR_DSH_SECRET_TOKEN_PATH};

/// Connect to DSH and retrieve the certificates and datastreams.json to create the properties struct
pub(crate) fn bootstrap(
    config_host: &str,
    tenant_name: &str,
    task_id: &str,
) -> Result<Cert, DshError> {
    let dsh_config = DshConfig::new(config_host, tenant_name, task_id)?;
    let client = reqwest_ca_client(dsh_config.dsh_ca_certificate.as_bytes())?;
    let dn = DshBootstapCall::Dn(&dsh_config).perform_call(&client)?;
    let dn = Dn::parse_string(&dn)?;
    let certificates = Cert::get_signed_client_cert(dn, &dsh_config, &client)?;
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

/// Helper struct to store the config needed for bootstrapping to DSH
#[derive(Debug)]
pub(crate) struct DshConfig<'a> {
    config_host: &'a str,
    tenant_name: &'a str,
    task_id: &'a str,
    dsh_secret_token: String,
    dsh_ca_certificate: String,
}
impl<'a> DshConfig<'a> {
    fn new(config_host: &'a str, tenant_name: &'a str, task_id: &'a str) -> Result<Self, DshError> {
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
        Ok(DshConfig {
            config_host,
            task_id,
            tenant_name,
            dsh_secret_token,
            dsh_ca_certificate,
        })
    }

    pub(crate) fn dsh_ca_certificate(&self) -> &str {
        &self.dsh_ca_certificate
    }
}

pub(crate) enum DshBootstapCall<'a> {
    /// Call to retreive distinguished name.
    Dn(&'a DshConfig<'a>),
    /// Call to post the certificate signing request.
    CertificateSignRequest {
        config: &'a DshConfig<'a>,
        csr: &'a str,
    },
}

impl DshBootstapCall<'_> {
    fn url_for_call(&self) -> String {
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
        let url = self.url_for_call();
        match self {
            DshBootstapCall::Dn(..) => client.get(url),
            DshBootstapCall::CertificateSignRequest { config, csr, .. } => client
                .post(url)
                .header("X-Kafka-Config-Token", &config.dsh_secret_token)
                .body(csr.to_string()),
        }
    }

    pub(crate) fn perform_call(&self, client: &Client) -> Result<String, DshError> {
        let response = self.request_builder(client).send()?;
        if !response.status().is_success() {
            return Err(DshError::DshCallError {
                url: self.url_for_call(),
                status_code: response.status(),
                error_body: response.text().unwrap_or_default(),
            });
        }
        Ok(response.text()?)
    }
}

/// Struct to parse DN string into separate fields.
/// Needed for Picky solution.
#[derive(Debug)]
pub(crate) struct Dn {
    cn: String,
    ou: String,
    o: String,
}

impl Dn {
    /// Parse the DN string into Dn struct.
    pub(crate) fn parse_string(dn_string: &str) -> Result<Self, DshError> {
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
    pub(crate) fn cn(&self) -> &str {
        &self.cn
    }

    pub(crate) fn ou(&self) -> &str {
        &self.ou
    }

    pub(crate) fn o(&self) -> &str {
        &self.o
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::str::from_utf8;

    #[test]
    fn test_dsh_call_request_builder() {
        let dsh_config = DshConfig {
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
        // create a DshConfig struct
        let dsh_config = DshConfig {
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
    #[serial(env_dependency)]
    fn test_dsh_config_new() {
        // normal situation where DSH variables are set
        env::set_var(VAR_DSH_SECRET_TOKEN, "test_token");
        env::set_var(VAR_DSH_CA_CERTIFICATE, "test_ca_certificate");
        let config_host = "https://test_host";
        let tenant_name = "test_tenant";
        let task_id = "test_task_id";
        let dsh_config = DshConfig::new(config_host, tenant_name, task_id).unwrap();
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
        let result = DshConfig::new(config_host, tenant_name, task_id);
        assert!(result.is_err());
        std::fs::write(test_token_dir.as_str(), "test_token_from_file").unwrap();
        let dsh_config = DshConfig::new(config_host, tenant_name, task_id).unwrap();
        assert_eq!(dsh_config.dsh_secret_token, "test_token_from_file");
        // fail if DSH_CA_CERTIFICATE is not set
        env::remove_var(VAR_DSH_CA_CERTIFICATE);
        let result = DshConfig::new(config_host, tenant_name, task_id);
        assert!(result.is_err());
        env::remove_var(VAR_DSH_SECRET_TOKEN);
        env::remove_var(VAR_DSH_CA_CERTIFICATE);
        env::remove_var(VAR_DSH_SECRET_TOKEN_PATH);
    }
}
