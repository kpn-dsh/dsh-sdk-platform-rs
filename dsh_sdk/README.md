
# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![dependency status](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs/status.svg)](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Rust SDK to interact with the DSH Platform. This library provides convenient building blocks for services that need to connect to DSH Kafka, fetch tokens for various protocols, manage Prometheus metrics, and more.

> **Note**  
> This library (v0.5.x) is a _release candidate_. It may contain incomplete features and/or bugs. Future updates might introduce breaking changes. Please report any issues you find.

---

## Table of Contents

1. [Migration Guide 0.4.X -> 0.5.X](#migration-guide-04x---05x)  
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

## Migration Guide 0.4.X -> 0.5.X

If you are migrating from `0.4.X` to `0.5.X`, please see the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for details on breaking changes and how to update your code accordingly.

---

## Description

The `dsh-sdk-platform-rs` library offers:

- **DSH Kafka Connectivity**  
  - Supports both direct DSH, Kafka Proxy, VPN, and local Kafka.  
  - Handles datastream information retrieval, certificate signing (bootstrap), and PKI configuration.

- **Token Fetchers**  
  - **Management API Token Fetcher**: For use with [`dsh_rest_api_client`](https://crates.io/crates/dsh_rest_api_client).  
  - **Protocol Token Fetcher**: Obtain tokens for MQTT and HTTP protocol adapters.

- **Kafka Configuration**  
  - RDKafka-based configuration, including vendored SSL support.

- **Common Utilities**  
  - Prometheus metrics (built-in HTTP server, plus re-export of the `metrics` crate).  
  - Tokio-based graceful shutdown handling.  
  - Dead Letter Queue (DLQ) functionality.

---

## Usage

To get started, add the following to your `Cargo.toml`:

```toml
[dependencies]
dsh_sdk = "0.5"
rdkafka = { version = "0.37", features = ["cmake-build", "ssl-vendored"] }
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

For more information, see the [CONNECT_PROXY_VPN_LOCAL.md](CONNECT_PROXY_VPN_LOCAL.md) document.

---

## Feature Flags

> **Important**  
> The feature flags have changed since the `v0.5.X` update. Check the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for details.

Below is an overview of the available features:

| **Feature**                  | **Default?** | **Description**                                                                      | **Example**                                                                            |
|------------------------------|--------------|--------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------|
| `bootstrap`                  | &check;      | Certificate signing process and datastreams property retrieval                       | [Kafka](./examples/kafka_example.rs) / [Kafka Proxy](./examples/kafka_proxy.rs)        |
| `kafka`                      | &check;      | Enables `DshKafkaConfig` trait and config structs for DSH Kafka connections          | [Kafka](./examples/kafka_example.rs) / [Kafka Proxy](./examples/kafka_proxy.rs)        |
| `rdkafka-config`             | &check;      | Provides a `DshKafkaConfig` implementation for RDKafka                               | [Kafka](./examples/kafka_example.rs) / [Kafka Proxy](./examples/kafka_proxy.rs)        |
| `schema-store`               | &cross;      | Interacts with the DSH Schema Store                                                 | [Schema Store API](./examples/schema_store_api.rs)                                    |
| `protocol-token-fetcher`     | &cross;      | Fetches tokens for DSH MQTT and HTTP protocol adapters                               | [Token fetcher](./examples/protocol_token_fetcher.rs) / [Specific claims](./examples/protocol_token_fetcher_specific_claims.rs) |
| `management-api-token-fetcher` | &cross;    | Fetches tokens for the DSH Management API                                           | [Token fetcher](./examples/management_api_token_fetcher.rs)                           |
| `metrics`                    | &cross;      | Adds Prometheus metrics (via an HTTP server) and re-exports `metrics` crate          | [Expose metrics](./examples/expose_metrics.rs) / [Custom metrics](./examples/custom_metrics.rs) |
| `graceful-shutdown`          | &cross;      | Adds a Tokio-based graceful shutdown handler                                        | [Graceful shutdown](./examples/graceful_shutdown.rs)                                  |
| `dlq`                        | &cross;      | Dead Letter Queue implementation                                                    | [Full example](./examples/dlq_implementation.rs)                                       |

### Selecting Features

To pick only the features you need, disable the default features and enable specific ones. For instance, if you only want the Management API Token Fetcher:

```toml
[dependencies]
dsh_sdk = { version = "0.5", default-features = false, features = ["management-api-token-fetcher"] }
```

---

## Environment Variables

This SDK uses certain environment variables to configure connections to DSH. For a full list of supported variables and their usage, see [ENV_VARIABLES.md](ENV_VARIABLES.md).

---

## Examples

You can find simple usage examples in the [`examples/` directory](./examples/).

### Full Service Example

A more complete example is provided in the [`example_dsh_service/`](../example_dsh_service/) directory, showcasing:

- How to build the Rust project
- How to package and push it to Harbor
- An end-to-end setup of a DSH service

See the [README](../example_dsh_service/README.md) in that directory for more information.

---

## Changelog

All changes per version are documented in [CHANGELOG.md](CHANGELOG.md).

---

## Contributing

Contributions are welcome! For details on how to help improve this project, please see [CONTRIBUTING.md](../CONTRIBUTING.md).

---

## License

This project is licensed under the [Apache License 2.0](../LICENSE).

---

## Security

For information about the security policy of this project, including how to report vulnerabilities, see [SECURITY.md](../SECURITY.md).

---

&copy; Koninklijke KPN N.V.

