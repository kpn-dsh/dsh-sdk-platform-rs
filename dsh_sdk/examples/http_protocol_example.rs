// run with 
// RUST_LOG=dsh_sdk=trace,reqwest=debug \
// cargo run --example http_protocol_example --features "protocol-token http-protocol-adapter"

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;

use dsh_sdk::protocol_adapters::token::data_access_token::{
    TopicPermission, Action, RequestDataAccessToken,
};

use dsh_sdk::Platform;

use dsh_sdk::protocol_adapters::http_protocol::{
    Accept, HttpClient, HttpConfig, HttpError,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ----- 1) Read required runtime configuration from env -----
    //
    // Required for token fetching (same as SDK examples):
    //  - API_KEY    : Your API client key (NEVER embed in clients)
    //  - TENANT     : e.g., "greenbox-dev"
    //  - CLIENT_ID  : any client id allowed by your token policy (e.g., "mees-local-test")
    //
    // Required for HTTP adapter:
    //  - PLATFORM_URL: e.g., "https://api.<platform-url>" for NpLz
    //  - STREAM      : e.g., "greenbox-test"
    //
    // Optional:
    //  - TOPIC       : e.g., "house/kitchen/sensor" (if present we do a real GET single)
    //
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let tenant_name = env::var("TENANT").expect("TENANT not set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let base_url = env::var("PLATFORM_URL").expect("PLATFORM_URL not set (https://api.<platform-url>)");
    let stream = env::var("STREAM").expect("STREAM not set (e.g., greenbox-test)");
    let topic = env::var("TOPIC").ok(); // optional

    // ----- 2) Fetch MQTT/HTTP Data Access Token (Platform = NpLz) -----
    //
    // This follows the SDK examples pattern; see the MQTT example that sets:
    //   const PLATFORM: dsh_sdk::Platform = dsh_sdk::Platform::NpLz;
    // and uses ApiClientTokenFetcher to fetch a DataAccessToken. [4](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/d4082f671027dbdb61cfdd0bebf54734105d17f6/dsh_sdk/examples/protocol_authentication_full_mediation.rs#L82C8-L82C39)
    //
    let platform = Platform::NpLz; // greenbox-dev lives on NpLz, not Poc
    let token_fetcher = ApiClientTokenFetcher::new(api_key, platform);

    env_logger::builder()
        .filter(Some("dsh_sdk"), log::LevelFilter::Trace)
        .filter(Some("reqwest"), log::LevelFilter::Debug) // optional but useful
        .target(env_logger::Target::Stdout)
        .init();

    // Define the permissions for the DataAccessToken
    // let permissions = vec![TopicPermission::new(
    //     Action::Subscribe,
    //     "amp",
    //     "/tt",
    //     format!("state/app/{}/#", tenant_name),
    // )];

    let permissions = vec![TopicPermission::new(
        Action::Subscribe,
        stream.clone(),
        "/tt",
        "#",
    )];

    // Create the token request
    let expiration_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_secs() as i64
        + 600; // 10 minutes in seconds

    // Request full-access token for this tenant/client_id.
    let req = RequestDataAccessToken::new(tenant_name.clone(), client_id.clone())
        .set_exp(expiration_time)
        .set_claims(permissions);

    let data_access_token = token_fetcher
        .fetch_data_access_token(req)
        .await?;
        // .expect("failed to fetch Data Access Token");

    // For HTTP, we must use the *data access token* value in Authorization:
    //   Authorization: Bearer <data_access_token>
    let bearer_value = data_access_token.raw_token().to_string();

    // ----- 3) Build our HTTP adapter config -----
    let cfg = HttpConfig::default()
        .with_base_url(base_url)
        .with_stream(stream)
        .with_mqtt_token(bearer_value) // we already store just the token string
        .with_accept(Accept::TextPlain)
        .with_timeout_secs(10);

    let client = HttpClient::new(&cfg)?;

    // ----- 4) No-topic probes (don’t require an existing topic) -----

    // HEAD probe to /data/v0/single — proves TLS, DNS, routing, and token acceptance
    client.check_connectivity(&cfg).await?;

    // ----- 5) Optional: real GET single if TOPIC is provided -----
    if let Some(t) = topic {
        match client.get_text_plain(&cfg, &t).await {
            Ok(body) => {
                println!("[GET single] stream={}, topic={} -> body: {}", cfg.stream, t, body);
            }
            Err(HttpError::Status(code)) => {
                eprintln!("[GET single] HTTP status {}", code);
            }
            Err(e) => {
                eprintln!("[GET single] error: {}", e);
            }
        }
    } else {
        println!("No TOPIC provided; skipped GET single call (as it requires a topic per spec).");
    }

    Ok(())
}
