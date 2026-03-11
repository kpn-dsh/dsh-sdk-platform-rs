//! Fully rewritten HTTP Protocol Adapter Example (v2)
//! Demonstrates:
//!   - POST retained
//!   - GET retained
//!   - DELETE retained
//!   - MULTI GET (wildcards +, #, and multiple filters)
//!
//! Uses: HttpClient v2 with Stream, Topic, Accept, ContentType models.

use std::env;

use dsh_sdk::protocol_adapters::http_protocol::{
    HttpClient, HttpClientBuilder, Stream, Topic,
    Accept, ContentType, ResponseBody, MultiGetItem,
};

use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
use dsh_sdk::protocol_adapters::token::data_access_token::{
    DataAccessToken, RequestDataAccessToken,
};

use dsh_sdk::Platform;

// ----------------------------
// PLATFORM SELECTION (KEPT EXACTLY THE SAME)
// ----------------------------
const PLATFORM: Platform = Platform::NpLz;

fn platform_base_url(platform: &Platform) -> String {
    platform
        .endpoint_management_api()
        .strip_suffix("/resources/v0")
        .unwrap_or(platform.endpoint_management_api())
        .to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("== HTTP POST + GET retained example using DSH (v2) ==");

    // ----------------------------
    // ENV VARIABLES (kept identical)
    // ----------------------------
    let stream_str = env::var("STREAM")
        .expect("STREAM environment variable is required");
    let topic_str = env::var("TOPIC").unwrap_or_else(|_| "".into());

    let stream = Stream::try_from(stream_str.as_str())?;
    let topic = Topic::try_from(topic_str.as_str())?;

    // ----------------------------
    // BASE URL FROM PLATFORM AS BEFORE
    // ----------------------------
    let base_url = platform_base_url(&PLATFORM);

    // ----------------------------
    // TOKEN RETRIEVAL (unchanged)
    // ----------------------------
    let mqtt_token_str = if let Ok(token) = env::var("MQTT_TOKEN") {
        token
    } else {
        let tenant_name = env::var("TENANT")?;
        let client_id = env::var("CLIENT_ID")?;

        let request = RequestDataAccessToken::new(tenant_name, client_id);
        let data_access_token = ApiClientAuthenticationService::get_data_access_token(request).await;
        data_access_token.raw_token().to_string()
    };

    println!("Using MQTT token: {}", mqtt_token_str);

    // ----------------------------
    // Build v2 HttpClient
    // ----------------------------
    let client = HttpClient::builder(&base_url)?
        .timeout(10)
        .build()?;

    // ##########################################################################
    // 1) FETCH EXISTING RETAINED MESSAGE (if exists)
    // ##########################################################################
    println!("\n--- STEP 1: Initial GET retained ---");

    match client.get_retained(&stream, &topic, Accept::TextPlain, &mqtt_token_str).await {
        Ok(body) => {
            print!("Existing retained message: ");
            match body {
                ResponseBody::Text(t) => println!("{t}"),
                ResponseBody::Bytes(b) => println!("(bytes) {:?}", b),
            }
        }
        Err(e) => {
            println!("No existing retained message (or error): {e}");
        }
    }

    // ##########################################################################
    // 2) POST RETAINED MESSAGE
    // ##########################################################################
    println!("\n--- STEP 2: POST retained message ---");

    // JSON payload for POST (goes as text/plain because DSH does not support json POST)
    let json_payload = r#"{"test": true, "value": 123456}"#;

    println!("Posting new JSON payload (as text/plain): {json_payload}");

    client.post_retained_body(
        &stream,
        &topic,
        ContentType::TextPlain,
        &mqtt_token_str,
        json_payload.as_bytes(),
    ).await?;

    println!("POST retained: OK ✓");

    // ##########################################################################
    // 3) GET TO VERIFY RETAINED UPDATE
    // ##########################################################################
    println!("\n--- STEP 3: GET retained after POST ---");

    let fetched = client.get_retained(&stream, &topic, Accept::TextPlain, &mqtt_token_str).await?;
    match fetched {
        ResponseBody::Text(t) => println!("Fetched retained: {t}"),
        ResponseBody::Bytes(b) => println!("Fetched retained (bytes): {:?}", b),
    }

    // ##########################################################################
    // 4) DELETE RETAINED MESSAGE
    // ##########################################################################
    println!("\n--- STEP 4: DELETE retained message ---");

    client.delete_retained(&stream, &topic, &mqtt_token_str).await?;
    println!("DELETE retained: OK ✓");

    // ##########################################################################
    // 5) MULTI-GET TEST DATA CREATION
    // ##########################################################################
    println!("\n--- STEP 5: Posting multiple sensor values for multi-get tests ---");

    client.post_retained_body(
        &stream, &Topic::try_from("sensors/temp/room1")?, 
        ContentType::TextPlain, &mqtt_token_str, b"21.5"
    ).await?;
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/temp/room2")?, 
        ContentType::TextPlain, &mqtt_token_str, b"22.1"
    ).await?;
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/humidity/room1")?, 
        ContentType::TextPlain, &mqtt_token_str, b"46"
    ).await?;
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/meta/info")?, 
        ContentType::TextPlain, &mqtt_token_str, b"metadata"
    ).await?;

    println!("Sensor test data posted ✓");

    // ##########################################################################
    // 6) MULTI GET WILDCARDS
    // ##########################################################################
    println!("\n=== STEP 6: MULTI-GET WILDCARD TESTS ===\n");

    let accept = Accept::TextPlain;

    // -------------------------------------
    println!("-- Exact match: sensors/temp/room1");
    let exact = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/temp/room1")?],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in exact {
        println!(" EXACT: {} => {}", item.topic, item.payload);
    }

    // -------------------------------------
    println!("\n-- + (single-level wildcard): sensors/temp/+");
    let plus = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/temp/+")?],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in plus {
        println!(" PLUS: {} => {}", item.topic, item.payload);
    }

    // -------------------------------------
    println!("\n-- # (multi-level wildcard): sensors/#");
    let hash = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/#")?],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in hash {
        println!(" HASH: {} => {}", item.topic, item.payload);
    }

    // -------------------------------------
    println!("\n-- Multiple filters: temp/+ and humidity/#");
    let multi_filters = client.multi_get(
        &stream,
        &[
            Topic::try_from("sensors/temp/+")?,
            Topic::try_from("sensors/humidity/#")?,
        ],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in multi_filters {
        println!(" MULTI: {} => {}", item.topic, item.payload);
    }

    // -------------------------------------
    println!("\n-- Empty result test: sensors/unknown/#");
    let empty = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/unknown/#")?],
        accept,
        &mqtt_token_str,
    ).await?;
    println!(" EMPTY RESULT OK — {} items returned", empty.len());

    println!("\n=== END OF MULTI-GET TESTS ===\n");

    Ok(())
}

// ---------------------------------------------------------------------
// TOKEN FETCHING (UNCHANGED AS YOU REQUESTED)
// ---------------------------------------------------------------------
struct ApiClientAuthenticationService;

impl ApiClientAuthenticationService {
    /// This should be properly implemented in your API Client Authentication service.
    async fn get_data_access_token(request: RequestDataAccessToken) -> DataAccessToken {
        let api_key = std::env::var("API_KEY").expect("API_KEY is not set");
        let token_fetcher = ApiClientTokenFetcher::new(api_key, PLATFORM);

        token_fetcher
            .fetch_data_access_token(request)
            .await
            .unwrap()
    }
}