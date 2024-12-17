pub(crate) mod config; // TODO: should we make this public? What benefits would that bring?

pub trait DshKafkaConfig {
    /// Set all required configurations to consume messages from DSH.
    fn dsh_consumer_config(&mut self) -> &mut Self;
    /// Set all required configurations to produce messages to DSH.
    fn dsh_producer_config(&mut self) -> &mut Self;
    /// Set a DSH compatible group id.
    ///
    /// DSH Requires a group id with the prefix of the tenant name.
    fn set_group_id(&mut self, group_id: &str) -> &mut Self;
}
