//! Example: Fetch a retained message over HTTP from DSH, using the Option‑B client.
//!
//! This example demonstrates two ways to supply the MQTT data‑access token:
//! 1) **Production path (default)** — provide `MQTT_TOKEN` via environment variable.
//!    The token should be minted by a trusted service or SDK component; do NOT mint it here.
//! 2) **Development path (feature `dev-token`)** — perform the REST→MQTT flow locally
//!    to obtain a token for quick, local testing. This path is gated behind a Cargo feature
//!    to discourage use in production builds.
//
// Run (production path):
//   $ export PLATFORM_URL="https://api.dsh-dev.dsh.np.aws.kpn.com"
//   $ export STREAM="reference-implementation"
//   $ export TOPIC="tt/reference-implementation/MQTT_Publish_test/retained"
//   $ export MQTT_TOKEN="<paste token from your trusted fetcher>"
//   $ RUST_LOG=info cargo run --example http_protocol_example --features "http-protocol-adapter"
//
// Run (development path: mint token inline; do NOT use in production):
//   # Requires: PLATFORM, TENANT, CLIENT_ID, API_KEY
//   $ export PLATFORM="api.dsh-dev.dsh.np.aws.kpn.com"
//   $ export TENANT="greenbox-dev"
//   $ export CLIENT_ID="mees-local-test"
//   $ export API_KEY="<your api key>"
//   # PLATFORM_URL and STREAM still required as in prod
//   $ export PLATFORM_URL="https://api.dsh-dev.dsh.np.aws.kpn.com"
//   $ export STREAM="reference-implementation"
//   $ export TOPIC="tt/reference-implementation/MQTT_Publish_test/retained"
//   $ RUST_LOG=info cargo run --example http_protocol_example --features "http-protocol-adapter,dev-token"
//
// Notes on variables:
// - PLATFORM_URL: Base URL used by the HTTP adapter for data retrieval (e.g., "https://api....").
// - PLATFORM:     Host used only by the dev-only token fetch helper (no scheme).
// - STREAM:       DSH stream name.
// - TOPIC:        Retained-message topic under the stream; if omitted, the GET is skipped.
// - MQTT_TOKEN:   Bearer token (data-access/MQTT token). Required in production path.
//
// Security: Never commit or log tokens/API keys. The logger is for demonstration only.

use dsh_sdk::protocol_adapters::http_protocol::{HttpClient, HttpConfig, Accept};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging from RUST_LOG (e.g., info, debug).
    env_logger::init();

    // --- Required configuration for the HTTP adapter itself ---
    let base_url = env::var("PLATFORM_URL")?; // e.g., "https://api.dsh-dev.dsh.np.aws.kpn.com"
    let stream   = env::var("STREAM")?;       // e.g., "reference-implementation"
    let topic    = env::var("TOPIC").ok();    // Optional: if None, we skip the GET

    // --- Select how to obtain the token ---
    // Production (default): expect MQTT_TOKEN from environment.
    // Development (feature "dev-token"): do REST->MQTT flow locally to get a token.
    #[cfg(feature = "dev-token")]
    let token = fetch_mqtt_token_via_rest_flow(
        &env::var("PLATFORM")?,   // e.g., "api.dsh-dev.dsh.np.aws.kpn.com" (host only, no scheme)
        &env::var("TENANT")?,     // e.g., "greenbox-dev"
        &env::var("CLIENT_ID")?,  // client id permitted by your token policy
        &env::var("API_KEY")?,    // API key — do NOT use outside dev scenarios
    ).await?;

    #[cfg(not(feature = "dev-token"))]
    let token = env::var("MQTT_TOKEN")?; // securely supplied by your trusted service/SDK

    // Build the adapter configuration (Option‑B: all required fields at construction).
    let cfg = HttpConfig::new(base_url, stream, token)
        .with_timeout_secs(10)
        .with_accept(Accept::TextPlain); // Default is TextPlain; shown here for clarity

    // Create the client. Internally it sets HTTPS-only and timeout. If you provided a custom CA
    // in the config (not shown here), the client would add it to the trust store.
    let client = HttpClient::new(cfg)?;

    // Perform a GET for the retained message if a TOPIC is provided.
    // Endpoint (constructed by the client): {base_url}/data/v0/single/tt/{stream}/{topic}
    if let Some(t) = topic {
        match client.get_text_plain(&t).await {
            Ok(body) => {
                println!("Retained message (text/plain):\n{}", body);
            }
            Err(e) => {
                // Typical failures:
                // - 401/403: token invalid or lacks permission for the stream/topic
                // - 404: no retained message exists at the given topic
                // - network/TLS issues: connectivity, DNS, or cert problems
                eprintln!("GET failed: {}", e);
            }
        }
    } else {
        println!("No TOPIC provided; skipping GET call.");
    }

    Ok(())
}

// --- Dev-only token helper ----------------------------------------------------
// This helper mints a MQTT (data-access) token using a REST->MQTT token flow.
// It is feature-gated to reduce the risk of shipping this path in production.
// cargo run --example http_protocol_example --features "http-protocol-adapter,dev-token"
#[cfg(feature = "dev-token")]
async fn fetch_mqtt_token_via_rest_flow(
    platform: &str,  // host only, e.g., "api.dsh-dev.dsh.np.aws.kpn.com"
    tenant: &str,
    client_id: &str,
    apikey: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    // 1) Obtain REST token
    let rest_url = format!("https://{}/auth/v0/token", platform);
    let rest_token_raw = client
        .post(rest_url)
        .header("apikey", apikey)
        .body(format!(r#"{{ "tenant": "{}" }}"#, tenant))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let rest_token = rest_token_raw.trim().trim_matches('"'); // token is often a quoted string

    // 2) Exchange for MQTT (data-access) token
    let mqtt_url = format!("https://{}/datastreams/v0/mqtt/token", platform);
    let mqtt_token_raw = client
        .post(mqtt_url)
        .bearer_auth(rest_token)
        .body(format!(r#"{{ "tenant": "{}", "id": "{}" }}"#, tenant, client_id))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(mqtt_token_raw.trim().to_owned())
}