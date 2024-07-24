# Environment Variables
The default RDKafka config can be overwritten by setting the following environment variables:

## Table of Contents
- [General](#general)
- [Consumer](#consumer)
- [Producer](#producer)

## General

### `KAFKA_BOOTSTRAP_SERVERS`
- Usage: Overwrite hostnames of brokers
- Default: Brokers based on datastreams
- Required: `false` (for local development or Kafka Proxy/VPN)

### `SCHEMA_REGISTRY_HOST`
- Usage: Overwrite Schema Registry host
- Default: Schema Registry based on datastreams
- Required: `false`

## Consumer

### `KAFKA_ENABLE_AUTO_COMMIT`
- Usage: Enable/Disable auto commit
- Default: `false`
- Required: `false`
- Options: `true`, `false`

### `KAFKA_AUTO_OFFSET_RESET`
- Usage: Set the offset reset settings to start consuming from set option.
- Default: earliest
- Required: `false`
- Options: smallest, earliest, beginning, largest, latest, end

### `KAFKA_CONSUMER_GROUP_TYPE`
- Usage: Picks group_id based on type from datastreams
- Default: Shared
- Options: private, shared
- Required: `false`

### `KAFKA_GROUP_ID`
- Usage: Custom group id
- Default: NA
- Required: `false`
- Remark: Overrules `KAFKA_CONSUMER_GROUP_TYPE`. Mandatory to start with tenant name. (will prefix tenant name automatically if not set)

## Producer

