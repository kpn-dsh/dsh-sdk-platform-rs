//! Demonstrates how an external client can use the SDK to connect to the DSH MQTT broker.
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, QoS};

use dsh_sdk::protocol_adapters::token_fetcher::*;

const PLATFORM: dsh_sdk::Platform = dsh_sdk::Platform::NpLz;

/// NEVER implement this logic in a device application/external clients!
/// 
/// This logic is part of API Client role in the DSH architecture, where the API Client 
/// delegates short lived tokens to devices with proper permissions. The API_KEY in this
/// code is the long lived REST token that the API Client uses to fetch short lived tokens 
/// for devices and this API_KEY should never be distributed
async fn get_protocol_token() -> ProtocolToken {
    let tenant_name = std::env::var("TENANT").unwrap().to_string();
    let api_key = std::env::var("API_KEY").unwrap().to_string();
    let mqtt_token_fetcher =
        ProtocolTokenFetcher::new(tenant_name, api_key, PLATFORM);
    mqtt_token_fetcher
        .get_token("Client-id", None) //Claims = None fetches all possible claims
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    let mut options = MqttOptions::new("Client-id", PLATFORM.endpoint_rest_api(), 1883);
    options.set_clean_session(true);

    let (mut client, notifications) = AsyncClient::new(options, 10);

    let token = get_protocol_token().await;

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        loop {
            let notification = rx.recv().await.unwrap();
            match notification {
                Event::Incoming(event) => {
                    println!("Incoming = {:?}", event);
                }
                Event::Outgoing(event) => {
                    println!("Outgoing = {:?}", event);
                }
            }
        }
    });

    client.subscribe("#", QoS::AtMostOnce).await.unwrap();

    loop {
        let notification = notifications.try_next();
        match notification {
            Ok(Some(notification)) => {
                tx.send(notification).await.unwrap();
            }
            Ok(None) => {
                break;
            }
            Err(e) => {
                println!("Error = {:?}", e);
                break;
            }
        }
    }
}
