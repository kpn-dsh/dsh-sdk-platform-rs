//! Lightweight HTTP client.
//!
//! This module provides a simple HTTP client to send requests to the server and other internal endpoints.
//!
//! # NOTE:
//! This client is not meant to be exposed as a public API, it is only meant to be used internally.

use hyper::Request;
use hyper_util::client::legacy::{connect::HttpConnector, Client};

use http::Uri;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_rustls::ConfigBuilderExt;
use hyper_util::rt::TokioExecutor;
use rustls::RootCertStore;
use std::str::FromStr;

async fn http_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://httpbin.org/ip";
    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>()?;
    if url.scheme_str() != Some("http") {
        eprintln!("This example only works with 'http' URLs.");
        return Ok(());
    }

    let client = Client::builder(TokioExecutor::new()).build(HttpConnector::new());

    let req = Request::builder()
        .uri(url)
        .body(Empty::<bytes::Bytes>::new())?;

    let resp = client.request(req).await?;

    eprintln!("{:?} {:?}", resp.version(), resp.status());
    eprintln!("{:#?}", resp.headers());

    Ok(())
}

async fn https_client() -> io::Result<()> {
    // Set a process wide default crypto provider.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let url = "http://httpbin.org/ip";

    // Second parameter is custom Root-CA store (optional, defaults to native cert store).
    let mut ca = match env::args().nth(2) {
        Some(ref path) => {
            let f = fs::File::open(path)
                .map_err(|e| error(format!("failed to open {}: {}", path, e)))?;
            let rd = io::BufReader::new(f);
            Some(rd)
        }
        None => None,
    };

    // Prepare the TLS client config
    let tls = match ca {
        Some(ref mut rd) => {
            // Read trust roots
            let certs = rustls_pemfile::certs(rd).collect::<Result<Vec<_>, _>>()?;
            let mut roots = RootCertStore::empty();
            roots.add_parsable_certificates(certs);
            // TLS client config using the custom CA store for lookups
            rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth()
        }
        // Default TLS client config with native roots
        None => rustls::ClientConfig::builder()
            .with_native_roots()?
            .with_no_client_auth(),
    };
    // Prepare the HTTPS connector
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http1()
        .build();

    // Build the hyper client from the HTTPS connector.
    let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new()).build(https);

    // Prepare a chain of futures which sends a GET request, inspects
    // the returned headers, collects the whole body and prints it to
    // stdout.
    let fut = async move {
        let res = client
            .get(url)
            .await
            .map_err(|e| error(format!("Could not get: {:?}", e)))?;
        println!("Status:\n{}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res
            .into_body()
            .collect()
            .await
            .map_err(|e| error(format!("Could not get body: {:?}", e)))?
            .to_bytes();
        println!("Body:\n{}", String::from_utf8_lossy(&body));

        Ok(())
    };

    fut.await
}
