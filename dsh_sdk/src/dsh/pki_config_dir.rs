//! Module for loading PKI Files from the $PKU_CONFIG_DIR directory.
//!
//! If for some reason the legacy bootstrap script is executed, these PKI files should be used,
//! instead of initiating a new PKI request.
//!
//! This also makes it possible to use the DSH SDK with Kafka Proxy
//! or VPN outside of the DSH environment.
use super::certificates::Cert;
use crate::error::DshError;
use crate::{utils, VAR_PKI_CONFIG_DIR};

use log::{debug, info, warn};
use pem::{self, Pem};
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
    info!("Certificates loaded from PKI config directory");
    Ok(Cert::new(
        pem::encode_many(&dsh_ca_certificate_pem),
        pem::encode_many(&dsh_client_certificate_pem),
        key_pair,
    ))
}

/// Get certificate from the PKI config directory
///
/// Looks for all files containing client*.pem and client.crt in the PKI config directory.
fn get_certificate(mut cert_paths: Vec<PathBuf>) -> Result<Vec<Pem>, DshError> {
    'file: while let Some(file) = cert_paths.pop() {
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
                    for p in &pem {
                        if !p.tag().eq_ignore_ascii_case("CERTIFICATE") {
                            warn!("{} - Certificate tag is not 'CERTIFICATE'", file.display());
                            continue 'file;
                        }
                    }
                    return Ok(pem);
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
    contains: PkiFileType,
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
                if filename.contains(prefix)
                    && match contains {
                        PkiFileType::Cert => {
                            filename.ends_with(".crt") || filename.ends_with(".pem")
                        }
                        PkiFileType::Key => filename.contains(".key"), //.key.pem is allowed
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

#[cfg(test)]
mod tests {
    use super::*;
    use openssl;
    use openssl::pkey::PKey;
    use serial_test::serial;

    const PKI_CONFIG_DIR: &str = "test_files/pki_config_dir";
    const PKI_KEY_FILE_NAME: &str = "client.key";
    const PKI_CERT_FILE_NAME: &str = "client.pem";
    const PKI_CA_FILE_NAME: &str = "ca.crt";

    fn create_test_pki_config_dir() {
        let path = PathBuf::from(PKI_CONFIG_DIR);
        let path_key = PathBuf::from(PKI_CONFIG_DIR).join(PKI_KEY_FILE_NAME);
        let path_cert = PathBuf::from(PKI_CONFIG_DIR).join(PKI_CERT_FILE_NAME);
        let path_ca = PathBuf::from(PKI_CONFIG_DIR).join(PKI_CA_FILE_NAME);
        if path_key.exists() && path_cert.exists() && path_ca.exists() {
            return;
        }
        let _ = std::fs::create_dir(path);
        let priv_key = openssl::rsa::Rsa::generate(2048).unwrap();
        let pkey = PKey::from_rsa(priv_key).unwrap();
        let key = pkey.private_key_to_pem_pkcs8().unwrap();
        let mut x509_name = openssl::x509::X509NameBuilder::new().unwrap();
        x509_name.append_entry_by_text("CN", "test_ca").unwrap();
        let x509_name = x509_name.build();
        let mut x509 = openssl::x509::X509::builder().unwrap();
        x509.set_version(2).unwrap();
        x509.set_subject_name(&x509_name).unwrap();
        x509.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0).unwrap())
            .unwrap();
        x509.set_not_after(&openssl::asn1::Asn1Time::days_from_now(365).unwrap())
            .unwrap();
        x509.set_pubkey(&pkey).unwrap();
        x509.sign(&pkey, openssl::hash::MessageDigest::sha256())
            .unwrap();
        let x509 = x509.build();
        let ca_cert = x509.to_pem().unwrap();
        let cert = x509.to_pem().unwrap();
        std::fs::write(path_key, key).unwrap();
        std::fs::write(path_ca, ca_cert).unwrap();
        std::fs::write(path_cert, cert).unwrap();
    }

    #[test]
    #[serial(pki)]
    fn test_get_file_path_bufs() {
        create_test_pki_config_dir();
        let path = PathBuf::from(PKI_CONFIG_DIR);
        let result_cert = get_file_path_bufs("client", PkiFileType::Cert, &path).unwrap();
        assert_eq!(result_cert.len(), 1);
        let result_key = get_file_path_bufs("client", PkiFileType::Key, &path).unwrap();
        assert_eq!(result_key.len(), 1);
        assert_ne!(result_cert, result_key);
        let result_ca = get_file_path_bufs("ca", PkiFileType::Cert, &path).unwrap();
        assert_eq!(result_ca.len(), 1);
        let result = get_file_path_bufs("ca", PkiFileType::Key, &path).unwrap();
        assert_eq!(result.len(), 0);
        let result = get_file_path_bufs("not_existing", PkiFileType::Key, &path).unwrap();
        assert_eq!(result.len(), 0);
    }
    #[test]
    #[serial(pki)]
    fn test_get_certificate() {
        create_test_pki_config_dir();
        let path_key = PathBuf::from(PKI_CONFIG_DIR).join(PKI_KEY_FILE_NAME);
        let path_cert = PathBuf::from(PKI_CONFIG_DIR).join(PKI_CERT_FILE_NAME);
        let path_ca = PathBuf::from(PKI_CONFIG_DIR).join(PKI_CA_FILE_NAME);
        let path_ne = PathBuf::from(PKI_CONFIG_DIR).join("not_existing.crt");
        let result = get_certificate(vec![path_cert.clone()]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tag(), "CERTIFICATE");
        let result = get_certificate(vec![path_ca.clone()]).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tag(), "CERTIFICATE");
        let result = get_certificate(vec![
            path_key.clone(),
            path_ne.clone(),
            path_cert.clone(),
            path_ca.clone(),
        ])
        .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tag(), "CERTIFICATE");
        let result = get_certificate(vec![path_key]).unwrap_err();
        assert!(matches!(result, DshError::NoCertificates));
        let result = get_certificate(vec![]).unwrap_err();
        assert!(matches!(result, DshError::NoCertificates));
        let result = get_certificate(vec![path_ne]).unwrap_err();
        assert!(matches!(result, DshError::NoCertificates));
    }

    #[test]
    #[serial(pki)]
    fn test_get_key_pair() {
        create_test_pki_config_dir();
        let path_key = PathBuf::from(PKI_CONFIG_DIR).join(PKI_KEY_FILE_NAME);
        let path_cert = PathBuf::from(PKI_CONFIG_DIR).join(PKI_CERT_FILE_NAME);
        let path_ca = PathBuf::from(PKI_CONFIG_DIR).join(PKI_CA_FILE_NAME);
        let path_ne = PathBuf::from(PKI_CONFIG_DIR).join("not_existing.key");
        let result = get_key_pair(vec![path_key.clone()]);
        assert!(result.is_ok());
        let result = get_key_pair(vec![path_ne.clone(), path_key.clone()]);
        assert!(result.is_ok());
        let result =
            get_key_pair(vec![path_ne.clone(), path_cert.clone(), path_ca.clone()]).unwrap_err();
        assert!(matches!(result, DshError::NoCertificates));
        let result = get_key_pair(vec![]).unwrap_err();
        assert!(matches!(result, DshError::NoCertificates));
        let result = get_key_pair(vec![path_ne]).unwrap_err();
        assert!(matches!(result, DshError::NoCertificates));
    }

    #[test]
    #[serial(pki, env_dependency)]
    fn test_get_pki_cert() {
        create_test_pki_config_dir();
        let result = get_pki_cert().unwrap_err();
        assert!(matches!(result, DshError::EnvVarError(_)));
        std::env::set_var(VAR_PKI_CONFIG_DIR, PKI_CONFIG_DIR);
        let result = get_pki_cert();
        assert!(result.is_ok());
        std::env::remove_var(VAR_PKI_CONFIG_DIR);
    }
}
