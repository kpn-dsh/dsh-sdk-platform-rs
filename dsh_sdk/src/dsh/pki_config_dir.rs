//! Module for loading PKI Files from the $PKU_CONFIG_DIR directory.
//!
//! If for some reason the legacy bootstrap script is executed, these PKI files should be used,
//! instead of initiating a new PKI request.
//!
//! This also makes it possible to use the DSH SDK with Kafka Proxy
//! or VPN outside of the DSH environment.
use super::certificates::Cert;
use super::{utils, VAR_PKI_CONFIG_DIR};
use crate::error::DshError;

use log::{debug, info, warn};
use pem;
use rcgen::KeyPair;
use std::path::{Path, PathBuf};

pub(crate) fn get_pki_cert() -> Result<Cert, DshError> {
    let config_dir = PathBuf::from(utils::get_env_var(VAR_PKI_CONFIG_DIR)?);
    let ca_cert_paths = get_file_path_bufs("ca", PkiFileType::Cert, &config_dir)?;
    let dsh_ca_certificate_pem = get_certificate(ca_cert_paths)?;
    let client_cert_paths = get_file_path_bufs("client", PkiFileType::Cert, &config_dir)?;
    let dsh_client_certificate_pem = get_certificate(client_cert_paths)?;
    let client_key_paths = get_file_path_bufs("client", PkiFileType::Key, &config_dir)?;
    let key_pair = get_key_pair(client_key_paths)?;
    debug!("Certificates loaded from PKI config directory");
    Ok(Cert::new(
        dsh_ca_certificate_pem,
        dsh_client_certificate_pem,
        key_pair,
    ))
}

/// Get certificate from the PKI config directory
///
/// Looks for all files containing client*.pem and client.crt in the PKI config directory.
fn get_certificate(mut cert_paths: Vec<PathBuf>) -> Result<String, DshError> {
    while let Some(file) = cert_paths.pop() {
        info!("{} - Reading certificate file", file.display());
        if let Ok(ca_cert) = std::fs::read(&file) {
            let pem_result = pem::parse_many(&ca_cert);
            match pem_result {
                Ok(pem) => {
                    debug!(
                        "{} - Certificate parsed as PEM ({} certificate in file)",
                        file.display(),
                        pem.len()
                    );
                    let pem_str = pem::encode_many(&pem);
                    return Ok(pem_str);
                }
                Err(e) => warn!("{} - Error parsing certificate: {:?}", file.display(), e),
            }
        }
    }
    info!("No (valid) certificates found in the PKI config directory");
    Err(DshError::NoCertificates)
}

/// Get certificate from the PKI config directory
///
/// Looks for all files containing client*.pem and client.crt in the PKI config directory.
fn get_key_pair(mut key_paths: Vec<PathBuf>) -> Result<KeyPair, DshError> {
    while let Some(file) = key_paths.pop() {
        info!("{} - Reading key file", file.display());
        if let Ok(bytes) = std::fs::read(&file) {
            if let Ok(string) = std::string::String::from_utf8(bytes) {
                debug!("{} - Key parsed as string", file.display());
                match rcgen::KeyPair::from_pem(&string) {
                    Ok(key_pair) => {
                        debug!("{} - Key parsed as KeyPair from string", file.display());
                        return Ok(key_pair);
                    }
                    Err(e) => warn!("{} - Error parsing key: {:?}", file.display(), e),
                }
            }
        }
    }
    info!("No (valid) key found in the PKI config directory");
    Err(DshError::NoCertificates)
}

/// Get the path to the PKI config direc
fn get_file_path_bufs<P>(
    prefix: &str,
    extension: PkiFileType,
    config_dir: P,
) -> Result<Vec<PathBuf>, DshError>
where
    P: AsRef<Path>,
{
    let file_paths: Vec<PathBuf> = config_dir
        .as_ref()
        .read_dir()?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let filename = e.file_name().to_string_lossy().into_owned();
                if filename.starts_with(prefix)
                    && match extension {
                        PkiFileType::Cert => {
                            filename.ends_with(".crt") || filename.ends_with(".pem")
                        }
                        PkiFileType::Key => filename.ends_with(".key"),
                    }
                {
                    Some(e.path())
                } else {
                    None
                }
            })
        })
        .collect();

    if file_paths.len() > 1 {
        warn!("Found multiple files: {:?}", file_paths);
    }

    Ok(file_paths)
}

/// Helper Enum for the type of PKI file
enum PkiFileType {
    Cert,
    Key,
}
