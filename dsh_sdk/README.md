
# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![dependency status](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs/status.svg)](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Rust SDK to interact with the DSH Platform. This library provides convenient building blocks for services that need to connect to DSH Kafka, fetch tokens for various protocols, manage Prometheus metrics, and more.

---

## Table of Contents

1. [Migration Guide](#migration-guide)  
2. [Description](#description)  
3. [Usage](#usage)  
4. [Connecting to DSH](#connect-to-dsh)  
5. [Feature Flags](#feature-flags)  
6. [Environment Variables](#environment-variables)  
7. [Examples](#examples)  
8. [Changelog](#changelog)  
9. [Contributing](#contributing)  
10. [License](#license)  
11. [Security](#security)  

---

## Migration Guide

If you are migrating from `v0.4.X`, please see the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for details on breaking changes and how to update your code accordingly.

---

## Description

The `dsh-sdk-platform-rs` library offers:

- **DSH Kafka Connectivity**  
  - Supports both direct DSH, Kafka Proxy, VPN, and local Kafka.  
  - Handles datastream information retrieval, certificate signing (bootstrap), and PKI configuration.

- **Token Fetchers**  
  - **Management API Token Fetcher**: For use with [`dsh_rest_api_client`](https://crates.io/crates/dsh_rest_api_client).  
  - **Protocol Token Fetcher**: Obtain tokens for MQTT and HTTP protocol adapters.

- **DSH Kafka Configuration**  
  - Trait for getting DSH Compatible Kafka Clients (DSH, Proxy, VPN and Local)
  - **RDKafka** implementation

- **Common Utilities**  
  - Lightweight HTTP server for exposing Metrics.  
  - Tokio-based graceful shutdown handling.  
  - Dead Letter Queue (DLQ) functionality.

---

## Usage

To get started, add the following to your `Cargo.toml`:

```toml
[dependencies]
dsh_sdk = "0.7"
rdkafka = { version = "0.38", features = ["cmake-build", "ssl-vendored"] }
```

> **Note**  
> By default, this SDK enables several features (see [Feature Flags](#feature-flags)). If you do not need them all, you can disable default features to reduce compile times and dependencies.

### Example

```rust
use dsh_sdk::DshKafkaConfig; // Trait for applying DSH-specific configurations
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure an rdkafka consumer with DSH settings
    let consumer: StreamConsumer = ClientConfig::new()
        .set_dsh_consumer_config()
        .create()?;

    // Your application logic here

    Ok(())
}
```

---

## Connect to DSH

This SDK accommodates multiple deployment environments:
- Running in a container on a DSH tenant
- Running in DSH System Space
- Running on a machine with Kafka Proxy/VPN
- Running locally with a local Kafka instance

For more information, see the [CONNECT_PROXY_VPN_LOCAL.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/CONNECT_PROXY_VPN_LOCAL.md) document.

---

## Feature Flags

> **Important**  
> The feature flags have changed since the `v0.5.X` update. Check the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for details.

Below is an overview of the available features:

| **feature**                    | **default** | **Description**                                                   | **Example**                                                                                                                          |
|--------------------------------|-------------|-------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| `bootstrap`                    | ✓           | Certificate signing process and fetch datastreams properties      | [Kafka](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_example.rs) / [Kafka Proxy](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_proxy.rs) |
| `kafka`                        | ✓           | Enable `DshKafkaConfig` trait and Config struct to connect to DSH | [Kafka](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_example.rs) / [Kafka Proxy](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_proxy.rs) |
| `rdkafka-config`               | ✓           | Enable `DshKafkaConfig` implementation for RDKafka                | [Kafka](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_example.rs) / [Kafka Proxy](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_proxy.rs) |
| `schema-store`                 | ✗           | Interact with DSH Schema Store                                    | [Schema Store API](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/schema_store_api.rs)                                                                                   |
| `protocol-token`       | ✗           | Fetch tokens to use DSH Protocol adapters (MQTT and HTTP)         | [Mqtt client](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/mqtt_example.rs) / [Mqtt websocket client](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/mqtt_example.rs) /<br>[Token fetcher (full mediation)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/protocol_authentication_full_mediation.rs) / [Token fetcher (partial mediation)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/protocol_authentication_partial_mediation.rs) |
| `management-api-token-fetcher` | ✗           | Fetch tokens to use DSH Management API                            | [Token fetcher](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/management_api_token_fetcher.rs)     |
| `metrics`                      | ✗           | Enable prometheus metrics including http server                   | [Expose metrics](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/expose_metrics.rs)                  |
| `graceful-shutdown`            | ✗           | Tokio based graceful shutdown handler                             | [Graceful shutdown](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/graceful_shutdown.rs)            |
| `dlq`                          | ✗           | Dead Letter Queue implementation                                  | [Full implementation example](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs) |

### Selecting Features

To pick only the features you need, disable the default features and enable specific ones. For instance, if you only want the Management API Token Fetcher:

```toml
[dependencies]
dsh_sdk = { version = "0.7", default-features = false, features = ["management-api-token-fetcher"] }
```

---

## Environment Variables

This SDK uses certain environment variables to configure connections to DSH. For a full list of supported variables and their usage, see [ENV_VARIABLES.md](ENV_VARIABLES.md).

---

## Examples

You can find simple usage examples in the [`examples/` directory](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/).

### Full Service Example

A more complete example is provided in the [`example_dsh_service/`](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/example_dsh_service/) directory, showcasing:

- How to build the Rust project
- How to package and push it to Harbor
- An end-to-end setup of a DSH service uising Kafka

See the [README](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/example_dsh_service/README.md) in that directory for more information.

---

## Changelog

All changes per version are documented in [CHANGELOG.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/CHANGELOG.md).

---

## Contributing

Contributions are welcome! For details on how to help improve this project, please see [CONTRIBUTING.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/CONTRIBUTING.md).

---

## License

This project is licensed under the [Apache License 2.0](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/LICENSE).

---

## Security

For information about the security policy of this project, including how to report vulnerabilities, see [SECURITY.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/SECURITY.md).

---

&copy; Koninklijke KPN N.V.

