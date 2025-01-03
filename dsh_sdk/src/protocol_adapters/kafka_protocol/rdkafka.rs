use rdkafka::ClientConfig;

use super::DshKafkaConfig;
use crate::Dsh;

impl DshKafkaConfig for ClientConfig {
    fn dsh_consumer_config(&mut self) -> &mut Self {
        let dsh = Dsh::get();
        let client_id = dsh.client_id();
        let config = dsh.kafka_config();

        self.set("bootstrap.servers", config.kafka_brokers())
            .set("group.id", config.group_id())
            .set("client.id", client_id)
            .set(
                "enable.auto.commit",
                config.enable_auto_commit().to_string(),
            )
            .set("auto.offset.reset", config.auto_offset_reset());
        if let Some(session_timeout) = config.session_timeout() {
            self.set("session.timeout.ms", session_timeout.to_string());
        }
        if let Some(queued_buffering_max_messages_kbytes) =
            config.queued_buffering_max_messages_kbytes()
        {
            self.set(
                "queued.max.messages.kbytes",
                queued_buffering_max_messages_kbytes.to_string(),
            );
        }
        log::debug!("Consumer config: {:#?}", self);
        self.set_dsh_certificates();
        self
    }

    fn dsh_producer_config(&mut self) -> &mut Self {
        let dsh = Dsh::get();
        let client_id = dsh.client_id();
        let config = dsh.kafka_config();
        self.set("bootstrap.servers", config.kafka_brokers())
            .set("client.id", client_id);
        if let Some(batch_num_messages) = config.batch_num_messages() {
            self.set("batch.num.messages", batch_num_messages.to_string());
        }
        if let Some(queue_buffering_max_messages) = config.queue_buffering_max_messages() {
            self.set(
                "queue.buffering.max.messages",
                queue_buffering_max_messages.to_string(),
            );
        }
        if let Some(queue_buffering_max_kbytes) = config.queue_buffering_max_kbytes() {
            self.set(
                "queue.buffering.max.kbytes",
                queue_buffering_max_kbytes.to_string(),
            );
        }
        if let Some(queue_buffering_max_ms) = config.queue_buffering_max_ms() {
            self.set("queue.buffering.max.ms", queue_buffering_max_ms.to_string());
        }
        log::debug!("Producer config: {:#?}", self);
        self.set_dsh_certificates();
        self
    }

    fn set_dsh_group_id(&mut self, group_id: &str) -> &mut Self {
        let tenant = Dsh::get().tenant_name();
        if group_id.starts_with(tenant) {
            self.set("group.id", group_id)
        } else {
            self.set("group.id", &format!("{}_{}", tenant, group_id))
        }
    }

    fn set_dsh_certificates(&mut self) -> &mut Self {
        let dsh = Dsh::get();
        if let Ok(certificates) = dsh.certificates() {
            self.set("security.protocol", "ssl")
                .set("ssl.key.pem", certificates.private_key_pem())
                .set(
                    "ssl.certificate.pem",
                    certificates.dsh_kafka_certificate_pem(),
                )
                .set("ssl.ca.pem", certificates.dsh_ca_certificate_pem())
        } else {
            self.set("security.protocol", "plaintext")
        }
    }
}
