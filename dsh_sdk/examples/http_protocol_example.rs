//! This example demonstrates how to use the DSH HTTP Protocol Adapter to publish, retrieve,
//! delete, and multi-get retained messages on the DSH platform.
//!
//! Example is using the following crates:
//! - [`dsh_sdk`] with features = ["http-protocol-adapter"] for using the HTTP protocol client
//! - [`tokio`] with features = ["full"] for async runtime
//! - [`env_logger`] for output logging to stdout to show what is happening
//!
//! NEVER distribute the API_KEY to an external client, this is only for demonstration purposes
//!
//! Run example with:
//! ```bash
//! STREAM={your_stream} TOPIC={your_topic} TENANT={your_tenant} CLIENT_ID={your_client_id} API_KEY={your_api_key} cargo run --example http_protocol_example --features "http-protocol-adapter protocol-token"
//! ```
//!
//! Alternatively, supply a pre-fetched token directly:
//! ```bash
//! STREAM={your_stream} TOPIC={your_topic} MQTT_TOKEN={your_token} cargo run --example http_protocol_example --features "http-protocol-adapter protocol-token"
//! ```
//!
//! The example will:
//! - Fetch a DataAccessToken (or use a pre-supplied MQTT_TOKEN)
//! - Build an HttpClient targeting the platform base URL
//! - GET an existing retained message on the given topic (if any)
//! - POST a new retained message to the given topic
//! - GET the retained message back to confirm the POST
//! - DELETE the retained message
//! - POST several sensor values to multiple topics
//! - Run a series of MULTI-GET requests using exact, single-level (+) and multi-level (#) wildcard filters
use std::env;
use std::time::Duration;

use dsh_sdk::protocol_adapters::http_protocol::{
    HttpClient, Stream, Topic,
    Accept, ContentType, ResponseBody,
};

use dsh_sdk::protocol_adapters::token::api_client_token_fetcher::ApiClientTokenFetcher;
use dsh_sdk::protocol_adapters::token::data_access_token::{
    DataAccessToken, RequestDataAccessToken,
};

use dsh_sdk::Platform;

// The platform to fetch the token for.
const PLATFORM: Platform = Platform::NpLz;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start logger to stdout to show what is happening
    env_logger::init();

    // Read stream and topic from environment variables
    let stream_str = env::var("STREAM")
        .expect("STREAM environment variable is required");
    let topic_str = env::var("TOPIC").unwrap_or_else(|_| "".into());

    let stream = Stream::try_from(stream_str)?;
    let topic = Topic::try_from(topic_str)?;

    let base_url = PLATFORM.http_protocol_base_url();

    // Use a pre-supplied token if available, otherwise fetch one via the API key
    let mqtt_token_str = if let Ok(token) = env::var("MQTT_TOKEN") {
        token
    } else {
        let tenant_name = env::var("TENANT")?;
        let client_id = env::var("CLIENT_ID")?;

        // Create a request for the Data Access Token
        let request = RequestDataAccessToken::new(tenant_name, client_id);

        // Fetch the token from your API Client Authentication service
        let data_access_token = ApiClientAuthenticationService::get_data_access_token(request).await;
        data_access_token.raw_token().to_string()
    };

    // Build the HTTP client targeting the platform base URL
    let client = HttpClient::builder(&base_url)?
        .timeout(Duration::from_secs(10))
        .build()?;

    println!("\n--- STEP 1: Initial GET ---
            [CALL] .get_retained() with
                    input:   stream='{}', topic='{}', accept=TextPlain\n",
                    stream.as_ref(),
                    topic.as_ref()
                );


    // Try to fetch any existing retained message; it may not exist yet, so errors are non-fatal
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

    println!("\n--- STEP 2: POST retained message ---
             [CALL] .post_retained_body() with
                     input:   stream='{}', topic='{}', content_type=TextPlain, payload='{{\"test\": true, \"value\": 123}}'\n",
                     stream.as_ref(),
                     topic.as_ref()
                 );

    // DSH does not support application/json as a POST content-type, so send as text/plain
    let json_payload = r#"{"test": true, "value": 123}"#;

    // Post a retained message with default QoS (1) and retained=true
    client.post_retained_body(
        &stream,
        &topic,
        ContentType::TextPlain,
        &mqtt_token_str,
        json_payload.as_bytes().to_vec(),
        None,
        None,
    ).await?;

    println!("POST retained: OK");
    println!("\n--- STEP 3: GET retained after using POST
            [CALL] .get_retained() with
                    input:   stream='{}', topic='{}', accept=TextPlain\n",
                    stream.as_ref(),
                    topic.as_ref()
                );

    // Fetch the retained message back to confirm the POST succeeded
    let fetched = client.get_retained(&stream, &topic, Accept::TextPlain, &mqtt_token_str).await?;
    match fetched {
        ResponseBody::Text(t) => println!("Fetched retained: {t}"),
        ResponseBody::Bytes(b) => println!("Fetched retained (bytes): {:?}", b),
    }

    println!("\n--- STEP 4: DELETE retained message ---
             [CALL] .delete_retained() with
                     input:   stream='{}', topic='{}'\n",
                     stream.as_ref(),
                     topic.as_ref()
                 );

    // Delete the retained message from the topic
    client.delete_retained(&stream, &topic, &mqtt_token_str).await?;
    println!("DELETE retained: OK");

    println!("\n--- STEP 5: Posting multiple sensor values for multi-get test ---");

    // Seed several topics with sensor data to use in the multi-get wildcard tests below
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/temp/room1")?, 
        ContentType::TextPlain, &mqtt_token_str, b"21.5".to_vec(), None, None
    ).await?;
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/temp/room2")?, 
        ContentType::TextPlain, &mqtt_token_str, b"22.1".to_vec(), None, None
    ).await?;
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/humidity/room1")?, 
        ContentType::TextPlain, &mqtt_token_str, b"46".to_vec(), None, None
    ).await?;
    client.post_retained_body(
        &stream, &Topic::try_from("sensors/meta/info")?, 
        ContentType::TextPlain, &mqtt_token_str, b"metadata".to_vec(), None, None
    ).await?;

    println!("✔ Seeded all test topics:
  sensors/temp/room1
  sensors/temp/room2
  sensors/humidity/room1
  sensors/meta/info");
    println!("\n=== STEP 6: MULTI-GET WILDCARD TESTS ===\n");

    // All multi-get calls in this example request text/plain payloads
    let accept = Accept::TextPlain;

    // Exact topic match — returns a single retained message
    println!("-- Exact match:
             [CALL] .multi_get() with
                     input:   stream='{}', topics=['sensors/temp/room1'], accept=TextPlain\n",
                     stream.as_ref()
                 );
    let exact = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/temp/room1")?],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in exact {
        println!("EXACT: {} => {}", item.topic, item.payload);
    }

    // Multiple exact filters in one request
    println!("\n-- Multiple filters:
             [CALL] .multi_get() with
                     input:   stream='{}', topics=['sensors/temp/room1', 'sensors/temp/room2'], accept=TextPlain\n",
                     stream.as_ref()
                 );
    let multi_filters = client.multi_get(
        &stream,
        &[
            Topic::try_from("sensors/temp/room1")?,
            Topic::try_from("sensors/temp/room2")?,
        ],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in multi_filters {
        println!(" MULTI: {} => {}", item.topic, item.payload);
    }

    // Single-level wildcard (+) — matches exactly one topic level
    println!("\n-- + (single-level wildcard):
             [CALL] .multi_get() with
                     input:   stream='{}', topics=['sensors/temp/+'], accept=TextPlain\n",
                     stream.as_ref()
                 );
    let plus = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/temp/+")?],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in plus {
        println!(" PLUS: {} => {}", item.topic, item.payload);
    }

    // Multi-level wildcard (#) — matches all topics under the given prefix
    println!("\n-- # (multi-level wildcard):
             [CALL] .multi_get() with
                     input:   stream='{}', topics=['sensors/#'], accept=TextPlain\n",
                     stream.as_ref()
                 );
    let hash = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/#")?],
        accept,
        &mqtt_token_str,
    ).await?;
    for item in hash {
        println!(" HASH: {} => {}", item.topic, item.payload);
    }

    // Combining different wildcard filters in one request
    println!("\n-- Multiple filters:
             [CALL] .multi_get() with
                     input:   stream='{}', topics=['sensors/temp/+', 'sensors/humidity/#'], accept=TextPlain\n",
                     stream.as_ref()
                 );
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

    // A filter that matches no retained messages should return an empty list, not an error
    println!("\n-- Empty result test:
             [CALL] .multi_get() with
                     input:   stream='{}', topics=['sensors/unknown/#'], accept=TextPlain\n",
                     stream.as_ref()
                 );
    let empty = client.multi_get(
        &stream,
        &[Topic::try_from("sensors/unknown/#")?],
        accept,
        &mqtt_token_str,
    ).await?;
    println!(" EMPTY RESULT OK — {} items returned", empty.len());

    println!("\n=== END OF TESTS ===\n");

    Ok(())
}

/// NEVER use this in an external client, this is only for demonstration purposes.
/// This should be implemented in your API Client Authentication service.
///
/// See the following examples for a more complete example:
/// - `examples/protocol_authentication_partial_mediation.rs`
/// - `examples/protocol_authentication_full_mediation.rs`
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