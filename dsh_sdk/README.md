# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![dependency status](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs/status.svg)](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Description
This library can be used to interact with the DSH Platform. It is intended to be used as a base for services that will be used to interact with DSH. It is not intended to be used directly. Features include:
- Connect to DSH 
- Fetch Kafka Properties and certificates
- Common functions 
  - Preconfigured RDKafka client config
  - Preconfigured Reqwest client config (for schema store)
- Graceful shutdown
- Prometheus Metrics (web server and re-export of metrics crate)
- Dead Letter Queue (experimental)

### Note
Rdkafka and thereby this library is dependent on CMAKE. Make sure it is installed in your environment and/or Dockerfile where you are compiling.
See [dockerfile](../example_dsh_service/Dockerfile) for an example.

## Usage
To use this SDK with the default features in your project, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = "0.4"
```

However, if you would like to use only specific features, you can specify them in your Cargo.toml file. For example, if you would like to use only the bootstrap feature, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = { version = "0.4", default-features = false, features = ["bootstrap"] }
```

See [feature flags](#feature-flags) for more information on the available features.

To use this SDK in your project
```rust
use dsh_sdk::Properties;
use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let dsh_properties = Properties::get();
    // get a rdkafka consumer config for example
    let consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config().create()?;
}
```

## Connect to DSH
The SDK is compatible with running in a container on a DSH tenant, on DSH System Space, on a machine with Kafka Proxy/VPN or on a local machine to a local Kafka. 
See [CONNECT_PROXY_VPN_LOCAL](/dsh_sdk/CONNECT_PROXY_VPN_LOCAL.md) for more info.

## Feature flags
The following features are available in this library and can be enabled/disabled in your Cargo.toml file.:

| **feature** | **default** | **Description** |
|---|---|---|
| `bootstrap` | &check; | Generate signed certificate and fetch datastreams info <br> Also makes certificates available, to be used as lowlevel API |
| `metrics` | &check; | Enable (custom) metrics for your service |
| `graceful_shutdown` | &check; | Create a signal handler for implementing a graceful shutdown |
| `dlq` | &cross; | Dead Letter Queue implementation (experimental) |
| `rdkafka-ssl` | &check; | Dynamically link to librdkafka to a locally installed OpenSSL |
| `rdkafka-ssl-vendored` | &cross; | Build OpenSSL during compile and statically link librdkafka <br> (No initial install required in environment, slower compile time) |

See api documentation for more information on how to use these features including.

## Environment variables
The default RDKafka config can be overwritten by setting the following environment variables:

### `KAFKA_BOOTSTRAP_SERVERS`
- Usage: Overwrite hostnames of brokers (useful for local testing)
- Default: Brokers based on datastreams
- Required: `false`

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

### `SCHEMA_REGISTRY_HOST`
- Usage: Overwrite Schema Registry host
- Default: Schema Registry based on datastreams
- Required: `false`

## Api doc
See the [api documentation](https://docs.rs/dsh_sdk/latest/dsh_sdk/) for more information on how to use this library.

## Examples
See folder [dsh_sdk/examples](/examples/) for simple examples on how to use the SDK.

### Full service example
See folder [example_dsh_service](../example_dsh_service/) for a full service, including how to build the Rust project and post it to Harbor. See [readme](../example_dsh_service/README.md) for more information.

## Changelog
See [CHANGELOG.md](CHANGELOG.md) for all changes per version.

## Contributing
See [CONTRIBUTING.md](../CONTRIBUTING.md) for more information on how to contribute to this project.

## License
See [LICENSE](../LICENSE) for more information on the license for this project.

## Security
See [SECURITY.md](../SECURITY.md) for more information on the security policy for this project.

---
_Copyright (c) Koninklijke KPN N.V._ 
