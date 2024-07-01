use std::time::Duration;

use log::{debug, trace};
use rumqttc::tokio_rustls::rustls::{ClientConfig, RootCertStore};
use rumqttc::{AsyncClient, EventLoop, MqttOptions, Transport};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DshError;

pub enum MqttEnv {
    Dev,
    Poc,
    Prod,
}

pub enum Port {
    Mqtts,   // MQTT over SSL / TLS
    Mqttwss, // MQTT over WebSockets - TODO: Will be implemented
}

#[derive(Serialize, Deserialize)]
pub struct MqttCredentials {
    pub mqtt_token: String,
    pub client_id: String,
}

pub struct BaseMqttOptions {
    mqtt_options: MqttOptions,
}

pub struct MqttClient {
    pub async_client: AsyncClient,
    pub eventloop: EventLoop,
}

impl BaseMqttOptions {
    pub async fn new(mqtt_env: MqttEnv) -> Self {
        let root_cert_store: RootCertStore = Self::get_tls_certificates().unwrap();
        let mqtt_options = Self::get_mqtt_options(
            Self::get_broker_url(mqtt_env),
            Self::get_port(Port::Mqtts),
            root_cert_store,
        )
        .await;
        BaseMqttOptions { mqtt_options }
    }
    fn get_broker_url(mqtt_env: MqttEnv) -> String {
        match mqtt_env {
            MqttEnv::Dev => "mqtt.dsh-dev.dsh.np.aws.kpn.com".to_string(),
            MqttEnv::Poc => "mqtt.poc.kpn-dsh.com".to_string(),
            MqttEnv::Prod => "mqtt.dsh-prod.dsh.np.aws.kpn.com".to_string(),
        }
    }
    fn get_port(port: Port) -> u16 {
        match port {
            Port::Mqtts => 8883,
            Port::Mqttwss => todo!(),
        }
    }
    async fn get_mqtt_options(
        broker_url: String,
        port: u16,
        root_cert_store: RootCertStore,
    ) -> MqttOptions {
        let device_id = Uuid::new_v4().to_string();
        let mut mqtt_options = MqttOptions::new(device_id, broker_url, port);
        mqtt_options.set_keep_alive(Duration::from_secs(60));
        let client_config = ClientConfig::builder()
            .with_root_certificates(root_cert_store.clone())
            .with_no_client_auth();
        debug!("client_config: {:?}", client_config);
        mqtt_options.set_transport(Transport::Tls(client_config.into()));
        debug!("mqtt options: {:?}", &mqtt_options);

        mqtt_options
    }
    fn get_tls_certificates() -> Result<RootCertStore, DshError> {
        let mut root_cert_store: RootCertStore = RootCertStore::empty();
        let certificates = rustls_native_certs::load_native_certs().unwrap();
        let (valid_count, invalid_count) = root_cert_store.add_parsable_certificates(certificates);
        trace!("Number of certificates added: {}", valid_count);
        trace!("Number of certificates ignored: {}", invalid_count);
        Ok(root_cert_store)
    }
}

impl MqttClient {
    pub async fn new(
        mut base_mqtt_options: BaseMqttOptions,
        credentials: MqttCredentials,
    ) -> MqttClient {
        base_mqtt_options
            .mqtt_options
            .set_credentials(credentials.client_id, credentials.mqtt_token);
        let (async_client, eventloop) = AsyncClient::new(base_mqtt_options.mqtt_options, 10);
        MqttClient {
            async_client,
            eventloop,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tls_certificates() {
        match BaseMqttOptions::get_tls_certificates() {
            Ok(root_cert_store) => {
                assert!(!root_cert_store.is_empty());
            }
            Err(_) => panic!("Failed to get TLS certificates"),
        }
    }

    #[tokio::test]
    async fn test_mqtt_client_publish() {
        let base_mqtt_options = BaseMqttOptions::new(MqttEnv::Dev).await;
        let credentials = MqttCredentials {
            mqtt_token: "test_token".to_string(),
            client_id: "test_client_id".to_string(),
        };
        let mqtt_client = MqttClient::new(base_mqtt_options, credentials).await;

        // Publish a test message
        let result = mqtt_client
            .async_client
            .publish(
                "test/topic",
                rumqttc::QoS::AtLeastOnce,
                false,
                "test_message",
            )
            .await;

        assert!(result.is_ok(), "Failed to publish message");
    }
}
