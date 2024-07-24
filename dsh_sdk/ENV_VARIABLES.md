# Environment Variables
The default RDKafka config can be overwritten by setting the following environment variables.
See also https://github.com/confluentinc/librdkafka/blob/master/CONFIGURATION.md for more information regarding LibRDKafka configuration.

## Table of Contents
- [General](#general)
- [Consumer](#consumer)
- [Producer](#producer)

## General

### `KAFKA_BOOTSTRAP_SERVERS`
- Usage: Overwrite hostnames of brokers
- RdKafka setting: `bootstrap.servers`
- Default: Brokers based on datastreams
- Required: `false` (for local development or Kafka Proxy/VPN)

### `SCHEMA_REGISTRY_HOST`
- Usage: Overwrite Schema Registry host url, which can be used to fetch schema
- RdKafka setting: N/A
- Default: Schema Registry based on datastreams
- Required: `false`

## Consumer
### `KAFKA_ENABLE_AUTO_COMMIT`
- Usage: Enable/Disable auto commit
- RdKafka setting: `enable.auto.commit`
- Default: `false`
- Required: `false`
- Options: `true`, `false`

### `KAFKA_AUTO_OFFSET_RESET`
- Usage: Set the offset reset settings to start consuming from set option.
- RdKafka setting: `auto.offset.reset`
- Default: earliest
- Required: `false`
- Options: smallest, earliest, beginning, largest, latest, end

### `KAFKA_CONSUMER_SESSION_TIMEOUT_MS`
- Usage: Set the session timeout in milliseconds
- RdKafka setting: `session.timeout.ms`
- Default: RDkafka default
- Required: `false`

### `KAFKA_CONSUMER_QUEUED_BUFFERING_MAX_MESSAGES_KBYTES`
- Usage: Maximum number of kilobytes of queued pre-fetched messages in the local consumer queue. 
- RdKafka setting: `queued.max.messages.kbytes`
- Default: RDkafka default
- Required: `false`

### `KAFKA_CONSUMER_GROUP_TYPE`
- Usage: Picks group_id based on type from datastreams
- Default: Shared
- Options: private, shared
- Required: `false`

### `KAFKA_GROUP_ID`
- Usage: Custom group id
- RdKafka setting: `group.id`
- Default: NA
- Required: `false`
- Remark: Overrules `KAFKA_CONSUMER_GROUP_TYPE`. Mandatory to start with tenant name. (will prefix tenant name automatically if not set)

## Producer

### `KAFKA_PRODUCER_BATCH_NUM_MESSAGES`
- Usage: Maximum number of messages batched in one message set
- RdKafka setting: `batch.num.messages`
- Default: RDkafka default
- Required: `false`


### `KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MESSAGES`
- Usage: Maximum number of messages allowed on the producer queue
- RdKafka setting: `queue.buffering.max.messages`
- Default: RDkafka default
- Required: `false`

### `KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_KBYTES`
- Usage: Maximum number of kilobytes allowed on the producer queue
- RdKafka setting: `queue.buffering.max.kbytes`
- Default: RDkafka default
- Required: `false`

### `KAFKA_PRODUCER_QUEUE_BUFFERING_MAX_MS`
- Usage: Maximum time the producer will wait for new messages before sending a message set
- RdKafka setting: `queue.buffering.max.ms`
- Default: RDkafka default
- Required: `false`
