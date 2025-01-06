pub mod config; // TODO: should we make this public? What benefits would that bring?

#[cfg(feature = "rdkafka")]
mod rdkafka;

pub trait DshKafkaConfig {
    /// Set all required configurations to consume messages from DSH Kafka Cluster.
    ///
    /// | **config**                | **Default value**                | **Remark**                                                             |
    /// |---------------------------|----------------------------------|------------------------------------------------------------------------|
    /// | `bootstrap.servers`       | Brokers based on datastreams     | Overwritable by env variable KAFKA_BOOTSTRAP_SERVERS`                  |
    /// | `group.id`                | Shared Group ID from datastreams | Overwritable by setting `KAFKA_GROUP_ID` or `KAFKA_CONSUMER_GROUP_TYPE`|
    /// | `client.id`               | Task_id of service               |                                                                        |
    /// | `enable.auto.commit`      | `false`                          | Overwritable by setting `KAFKA_ENABLE_AUTO_COMMIT`                     |
    /// | `auto.offset.reset`       | `earliest`                       | Overwritable by setting `KAFKA_AUTO_OFFSET_RESET`                      |
    /// | `security.protocol`       | ssl (DSH) / plaintext (local)    | Security protocol                                                      |
    /// | `ssl.key.pem`             | private key                      | Generated when sdk is initiated                                        |
    /// | `ssl.certificate.pem`     | dsh kafka certificate            | Signed certificate to connect to kafka cluster                         |
    /// | `ssl.ca.pem`              | CA certifacte                    | CA certificate, provided by DSH.                                       |
    fn set_dsh_consumer_config(&mut self) -> &mut Self;
    /// Set all required configurations to produce messages to DSH Kafka Cluster.
    ///
    /// ## Configurations
    /// | **config**          | **Default value**              | **Remark**                                                                              |
    /// |---------------------|--------------------------------|-----------------------------------------------------------------------------------------|
    /// | bootstrap.servers   | Brokers based on datastreams   | Overwritable by env variable `KAFKA_BOOTSTRAP_SERVERS`                                  |
    /// | client.id           | task_id of service             | Based on task_id of running service                                                     |
    /// | security.protocol   | ssl (DSH)) / plaintext (local) | Security protocol                                                                       |
    /// | ssl.key.pem         | private key                    | Generated when bootstrap is initiated                                                   |
    /// | ssl.certificate.pem | dsh kafka certificate          | Signed certificate to connect to kafka cluster <br>(signed when bootstrap is initiated) |
    /// | ssl.ca.pem          | CA certifacte                  | CA certificate, provided by DSH.                                                        |
    fn set_dsh_producer_config(&mut self) -> &mut Self;
    /// Set a DSH compatible group id.
    ///
    /// DSH Requires a group id with the prefix of the tenant name.
    fn set_dsh_group_id(&mut self, group_id: &str) -> &mut Self;
    /// Set the required DSH Certificates.
    ///
    /// This function will set the required SSL configurations if the certificates are present.
    /// Else it will return plaintext. (for connection to a local kafka cluster)
    fn set_dsh_certificates(&mut self) -> &mut Self;
}
