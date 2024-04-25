//! Module for bootstrapping the DSH client.
//!
//! This module contains the logic to connect to DSH and retrieve the certificates and datastreams.json
//! to create the properties struct. It follows the certificate signing request pattern as normally
//! used in the get_signed_certificates_json.sh script.
//!
//! ## Note
//!
//! This module is not intended to be used directly, but through the `Properties` struct. It will
//! always be used when getting a `Properties` struct vja dsh::Properties::get().
//!
//! If this module returns an error, it defaults to the local_datastreams.json file, so it can be used
//! in a local environment. (when feature `local` is enabled)
//!
//! ## Example
//! ```
//! use dsh_sdk::dsh::Properties;
//!
//! let dsh_properties = Properties::get();
//! ```

use log::{debug, info, warn};
use reqwest::blocking::Client;

use std::env;

use crate::error::DshError;

use super::{certificates::Cert, datastream::Datastream, Properties};

impl Properties {
    /// Connect to DSH and retrieve the certificates and datastreams.json to create the properties struct
    pub(crate) fn new_dsh() -> Result<Self, DshError> {
        let dsh_config = DshConfig::new()?;
        let client = Properties::reqwest_client(dsh_config.dsh_ca_certificate.as_bytes())?;
        let dn = DshCall::Dn(&dsh_config).perform_call(&client)?;
        let dn = Dn::parse_string(&dn)?;
        let certificates = Cert::new(dn, &dsh_config, &client)?;
        let client_with_cert = certificates.reqwest_blocking_client_config()?.build()?;
        let datastreams_string =
            DshCall::Datastream(&dsh_config).perform_call(&client_with_cert)?;
        let datastream: Datastream = serde_json::from_str(&datastreams_string)?;
        Ok(Self {
            client_id: dsh_config.task_id.to_string(),
            tenant_name: dsh_config.tenant_name.to_string(),
            task_id: dsh_config.task_id.to_string(),
            datastream,
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

#[derive(Debug)]
pub(crate) struct DshConfig {
    config_host: String,
    tenant_name: String,
    task_id: String,
    dsh_secret_token: String,
    dsh_ca_certificate: String,
}

/// Helper struct to store the config needed for bootstrapping to DSH
impl DshConfig {
    fn new() -> Result<Self, DshError> {
        let config_host = Self::get_env_var("KAFKA_CONFIG_HOST")
            .map(|host| format!("https://{}", host))
            .unwrap_or_else(|_| {
                let default = "https://pikachu.dsh.marathon.mesos:4443".to_string();
                warn!(
                    "KAFKA_CONFIG_HOST is not set, using default value {}",
                    default
                );
                default
            });

        let task_id = Self::get_env_var("MESOS_TASK_ID")?;
        let app_id = Self::get_env_var("MARATHON_APP_ID")?;
        let dsh_secret_token = match Self::get_env_var("DSH_SECRET_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                // if DSH_SECRET_TOKEN is not set, try to read it from a file (for system space applications)
                info!("trying to read DSH_SECRET_TOKEN from file");
                let secret_token_path = Self::get_env_var("DSH_SECRET_TOKEN_PATH")?;
                let path = std::path::PathBuf::from(secret_token_path);
                std::fs::read_to_string(path)?
            }
        };
        let dsh_ca_certificate = Self::get_env_var("DSH_CA_CERTIFICATE")?;
        let tenant_name = DshConfig::tenant_name(&app_id);
        Ok(DshConfig {
            config_host,
            task_id,
            tenant_name: tenant_name.to_string(),
            dsh_secret_token,
            dsh_ca_certificate,
        })
    }

    pub(crate) fn dsh_ca_certificate(&self) -> &str {
        &self.dsh_ca_certificate
    }

    /// Derive the tenant name from the app id.
    fn tenant_name(app_id: &str) -> &str {
        let tenant_name = app_id.split('/').nth(1);
        match tenant_name {
            Some(tenant_name) => tenant_name,
            None => {
                warn!(
                    "MARATHON_APP_ID is not as expected, missing expected slashes, using \"{}\" as tenant name",
                    app_id
                );
                app_id
            }
        }
    }

    fn get_env_var(var_name: &str) -> Result<String, DshError> {
        debug!("Reading {} from environment variable", var_name);
        match env::var(var_name) {
            Ok(value) => Ok(value),
            Err(e) => {
                warn!("{} is not set", var_name);
                Err(e.into())
            }
        }
    }
}

pub(crate) enum DshCall<'a> {
    /// Call to retreive distinguished name.
    Dn(&'a DshConfig),
    /// Call to retreive datastreams.json.
    Datastream(&'a DshConfig),
    /// Call to post the certificate signing request.
    CertificateSignRequest { config: &'a DshConfig, csr: &'a str },
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

    fn request_builder(&self, url: &str, client: &Client) -> reqwest::blocking::RequestBuilder {
        match self {
            DshCall::Dn(..) | DshCall::Datastream(..) => client.get(url),
            DshCall::CertificateSignRequest { config, csr, .. } => client
                .post(url)
                .header("X-Kafka-Config-Token", &config.dsh_secret_token)
                .body(csr.to_string()),
        }
    }

    pub(crate) fn perform_call(&self, client: &Client) -> Result<String, DshError> {
        let url = self.url_for_call();
        let response = self.request_builder(&url, client).send()?;
        if !response.status().is_success() {
            return Err(DshError::DshCallError {
                url,
                status_code: response.status(),
                error_body: response.text()?,
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
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn test_dsh_config_tenant_name() {
        let app_id = "/greenbox-dev/app-name";
        let result = DshConfig::tenant_name(app_id);
        assert_eq!(
            result,
            "greenbox-dev".to_string(),
            "{} is not parsed correctly",
            app_id
        );
        let app_id = "greenbox-dev";
        let result = DshConfig::tenant_name(app_id);
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
        let builder: reqwest::blocking::RequestBuilder =
            DshCall::Dn(&dsh_config).request_builder("https://test_host", &Client::new());
        let request = builder.build().unwrap();
        assert_eq!(request.method().as_str(), "GET");
        let builder: reqwest::blocking::RequestBuilder =
            DshCall::Datastream(&dsh_config).request_builder("https://test_host", &Client::new());
        let request = builder.build().unwrap();
        assert_eq!(request.method().as_str(), "GET");
        let csr = "-----BEGIN test_type-----\n-----END test_type-----";
        let builder: reqwest::blocking::RequestBuilder = DshCall::CertificateSignRequest {
            config: &dsh_config,
            csr,
        }
        .request_builder("https://test_host", &Client::new());
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
            config_host: dsh.url(),
            tenant_name: "tenant".to_string(),
            task_id: "test_task_id".to_string(),
            dsh_secret_token: "test_token".to_string(),
            dsh_ca_certificate: "test_ca_certificate".to_string(),
        };
        // call the function
        let response = DshCall::Dn(&dsh_config).perform_call(&client).unwrap();
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
    fn test_get_env_var() {
        env::set_var("TEST_ENV_VAR", "test_value");
        let result = DshConfig::get_env_var("TEST_ENV_VAR").unwrap();
        assert_eq!(result, "test_value");
    }

    #[test]
    fn test_dsh_config_new() {
        // normal situation where DSH variables are set
        env::set_var("KAFKA_CONFIG_HOST", "test_host");
        env::set_var("MESOS_TASK_ID", "test_task_id");
        env::set_var("MARATHON_APP_ID", "/test_tenant/test_app");
        env::set_var("DSH_SECRET_TOKEN", "test_token");
        env::set_var("DSH_CA_CERTIFICATE", "test_ca_certificate");
        let dsh_config = DshConfig::new().unwrap();
        assert_eq!(dsh_config.config_host, "https://test_host");
        assert_eq!(dsh_config.task_id, "test_task_id");
        assert_eq!(dsh_config.tenant_name, "test_tenant");
        assert_eq!(dsh_config.dsh_secret_token, "test_token");
        assert_eq!(dsh_config.dsh_ca_certificate, "test_ca_certificate");
        // DSH_SECRET_TOKEN is not set, but DSH_SECRET_TOKEN_PATH is set
        env::remove_var("DSH_SECRET_TOKEN");
        let test_token_dir = "test_files";
        std::fs::create_dir_all(test_token_dir).unwrap();
        let test_token_dir = format!("{}/test_token", test_token_dir);
        let _ = std::fs::remove_file(&test_token_dir);
        env::set_var("DSH_SECRET_TOKEN_PATH", &test_token_dir);
        let result = DshConfig::new();
        assert!(result.is_err());
        std::fs::write(test_token_dir.as_str(), "test_token_from_file").unwrap();
        let dsh_config = DshConfig::new().unwrap();
        assert_eq!(dsh_config.dsh_secret_token, "test_token_from_file");
        // fail if DSH_CA_CERTIFICATE is not set
        env::remove_var("DSH_CA_CERTIFICATE");
        let result = DshConfig::new();
        assert!(result.is_err());
    }
}
