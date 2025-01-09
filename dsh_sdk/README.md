# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![dependency status](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs/status.svg)](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

# NOTE
As this is a release candidate it may contain bugs and/or incomplete features and incorrect documentation and future updates may contain breaking changes.

Please report any issues you encounter.

## Migration guide 0.4.X -> 0.5.X
See [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for more information on how to migrate from 0.4.X to 0.5.X.

## Description
This library can be used to interact with the DSH Platform. It is intended to be used as a base for services that will be used to interact with DSH. Features include:
- Connect to DSH Kafka (DSH, Kafka Proxy, VPN, System Space, Local)
  - Bootstrap (fetch datastreams info and generate signed certificate)
  - PKI Config Directory (for Kafka Proxy/VPN)
- Kafka config for DSH (incl. RDKafka)
- Management API Token Fetcher (to be used with [dsh_rest_api_client](https://crates.io/crates/dsh_rest_api_client))
- Protocol Token Fetcher (MQTT and HTTP)
- Common utilities 
  - Prometheus Metrics (web server and re-export of metrics crate)
  - Graceful shutdown
  - Dead Letter Queue 

## Usage
To use this SDK with the default features in your project, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = "0.5"
rdkafka = { version =  "0.37", features = ["cmake-build", "ssl-vendored"] }
```
See [feature flags](#feature-flags) for more information on the available features.

To use this SDK in your project
```rust
use dsh_sdk::DshKafkaConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // get a rdkafka consumer config for example
    let consumer: StreamConsumer = ClientConfig::new().set_dsh_consumer_config().create()?;
    Ok(())
}
```

## Connect to DSH
The SDK is compatible with running in a container on a DSH tenant, on DSH System Space, on a machine with Kafka Proxy/VPN or on a local machine to a local Kafka. 
See [CONNECT_PROXY_VPN_LOCAL](CONNECT_PROXY_VPN_LOCAL.md) for more info.

## Feature flags
See the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for more information on the changes in feature flags since the v0.5.X update.

The following features are available in this library and can be enabled/disabled in your Cargo.toml file:

| **feature** | **default** | **Description** | **Example** |
| --- |--- | --- | --- |
| `bootstrap` | &check; | Certificate signing process and fetch datastreams properties |  [Kafka](./examples/kafka_example.rs) / [Kafka Proxy](./examples/kafka_proxy.rs) |
| `kafka` |  &check; | Enable `DshKafkaConfig` trait and Config struct to connect to DSH |  [Kafka](./examples/kafka_example.rs) / [Kafka Proxy](./examples/kafka_proxy.rs) |
| `rdkafka-config` | &check; | Enable `DshKafkaConfig` implementation for RDKafka | [Kafka](./examples/kafka_example.rs) / [Kafka Proxy](./examples/kafka_proxy.rs) |
| `schema-store` | &cross; | Interact with DSH Schema Store | [Schema Store API](./examples/schema_store_api.rs) |
| `protocol-token-fetcher` | &cross; | Fetch tokens to use DSH Protocol adapters (MQTT and HTTP) | [Token fetcher](./examples/protocol_token_fetcher.rs) / [with specific claims](./examples/protocol_token_fetcher_specific_claims.rs) |
| `management-api-token-fetcher` | &cross; | Fetch tokens to use DSH Management API | [ Token fetcher](./examples/management_api_token_fetcher.rs) |
| `metrics` | &cross; | Enable prometheus metrics including http server | [Expose metrics](./examples/expose_metrics.rs) / [Custom metrics](./examples/custom_metrics.rs) |
| `graceful-shutdown` | &cross; | Tokio based graceful shutdown handler | [Graceful shutdown](./examples/graceful_shutdown.rs) |
| `dlq` | &cross; | Dead Letter Queue implementation | [Full implementation example](./examples/dlq_implementation.rs) |

See the [api documentation](https://docs.rs/dsh_sdk/latest/dsh_sdk/) for more information on how to use these features.

If you would like to use specific features, you can specify them in your Cargo.toml file. This can save compile time and dependencies.
For example, if you only want to use the Management API token fetcher feature, add the following to your Cargo.toml file:

```toml
[dependencies]
dsh_sdk = { version = "0.5", default-features = false, features = ["management-api-token-fetcher"] }
```

## Environment variables
The SDK checks environment variables to change configuration for connnecting to DSH.
See [ENV_VARIABLES.md](ENV_VARIABLES.md)  which .

## Examples
See folder [dsh_sdk/examples](./examples/) for simple examples on how to use the SDK.

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
