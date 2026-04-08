
# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![dependency status](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs/status.svg)](https://deps.rs/repo/github/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A Rust SDK to interact with the DSH Platform. This library provides convenient building blocks for services that need to connect to DSH Kafka, fetch tokens for various protocols, manage Prometheus metrics, and more.

---

## Table of Contents

1. [Description](#description)  
2. [Usage](#usage)  
3. [Connecting to DSH](#connect-to-dsh)  
4. [Feature Flags](#feature-flags)  
5. [Environment Variables](#environment-variables)  
6. [Examples](#examples)  
7. [Changelog](#changelog)  
8. [Contributing](#contributing)  
9. [License](#license)  
10. [Security](#security)  
11. [Migration Guide](#migration-guide) 

---

## Description

The `dsh-sdk-platform-rs` library offers:

- **DSH Kafka Connectivity**  
  - Trait for getting DSH Compatible Kafka Clients (DSH, Proxy, VPN and Local)
    - **RDKafka** implementation included
  - Handles datastream information retrieval, certificate signing (bootstrap), and PKI configuration.

- **Certificates**
  - Sign certificates which can be used for secure communication with DSH Kafka and (m)TLS transport between containers.

- **Token Fetchers**  
  - **Management API Token Fetcher**: For use with [`dsh_rest_api_client`](https://crates.io/crates/dsh_rest_api_client).  
  - **Protocol Token Fetcher**: Obtain tokens for MQTT and HTTP protocol adapters.

- **Schema Store Interaction**  
  - Fetch and manage schema from the DSH Schema Store.
  
- **Common Utilities**  
  - Lightweight HTTP server for exposing Metrics.  
  - Tokio-based graceful shutdown handling.  
  - Dead Letter Queue (DLQ) functionality.

---

## Usage

To get started, add the following to your `Cargo.toml`:

```toml
[dependencies]
dsh_sdk = "0.8"
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
This SDK supports kafka, MQTT and HTTP connectivity to DSH. Depending on your environment and use case, you can choose the appropriate connection method. 

### DSH Kafka Connectivity
This SDK accommodates multiple environments to connect to DSH Kafka, including:
- Running in a container on a DSH tenant
- Running in DSH System Space
- Running on a machine with Kafka Proxy/VPN
- Running locally with a local Kafka instance

For more information, see the [CONNECT_PROXY_VPN_LOCAL.md](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/CONNECT_PROXY_VPN_LOCAL.md) document.

### MQTT Protocol Adapter
To connect to DSH using MQTT, you can use the Protocol Token Fetcher to obtain the necessary authentication tokens. 
This allows you to interact with DSH's MQTT protocol adapters securely. 

We recommend using the [`rumqttc`](https://crates.io/crates/rumqttc) crate for MQTT connectivity in Rust, which can be configured with the tokens obtained from the SDK.

For more details, see the [Mqtt client](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/mqtt_example.rs) / [Mqtt websocket client](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/mqtt_example.rs) examples.

> **Note**
> Always make sure to handle your API KEY correctly and NEVER use it directly in client-side applications. Use the token fetcher to obtain short-lived tokens for authentication instead and delegate the responsibility of token management to your backend services.

### HTTP Protocol Adapter

Similar to MQTT, you can use the Protocol Token Fetcher to obtain tokens for authenticating with DSH's HTTP protocol adapters. This allows you to send HTTP requests to DSH services securely. This SDK provides a client in the `http_protocol_adapter` module to facilitate interactions with the HTTP Protocol Adapter.

An example of how to use the HTTP Protocol Adapter client can be found in the [HTTP Protocol Adapter example](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/http_protocol_example.rs).

> **Note**
> Always make sure to handle your API KEY correctly and NEVER use it directly in client-side applications. Use the token fetcher to obtain short-lived tokens for authentication instead and delegate the responsibility of token management to your backend services.


---

## Feature Flags

Below is an overview of the available features:

| **feature**                    | **default** | **Description**                                                   | **Example**                                                                                                                          |
|--------------------------------|-------------|-------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------|
| `bootstrap`                    | ✓           | Certificate signing process and fetch datastreams properties      | [Kafka](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_example.rs) / [Kafka Proxy](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_proxy.rs) |
| `kafka`                        | ✓           | Enable `DshKafkaConfig` trait and Config struct to connect to DSH | [Kafka](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_example.rs) / [Kafka Proxy](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_proxy.rs) |
| `rdkafka-config`               | ✓           | Enable `DshKafkaConfig` implementation for RDKafka                | [Kafka](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_example.rs) / [Kafka Proxy](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/kafka_proxy.rs) |
| `schema-store`                 | ✗           | Interact with DSH Schema Store                                    | [Schema Store API](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/schema_store_api.rs)                                                                                   |
| `protocol-token`       | ✗           | Fetch tokens to use DSH Protocol adapters (MQTT and HTTP)         | [Mqtt client](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/mqtt_example.rs) / [Mqtt websocket client](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/mqtt_example.rs) /<br>[Token fetcher (full mediation)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/protocol_authentication_full_mediation.rs) / [Token fetcher (partial mediation)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/protocol_authentication_partial_mediation.rs) |
| `http-protocol-adapter`        | ✗           | HTTP client to interact with DSH HTTP Protocol Adapter            | [HTTP Protocol Adapter example](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/http_protocol_example.rs) |
| `management-api-token-fetcher` | ✗           | Fetch tokens to use DSH Management API                            | [Token fetcher](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/management_api_token_fetcher.rs)     |
| `metrics`                      | ✗           | Enable prometheus metrics including http server                   | [Expose metrics](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/expose_metrics.rs)                  |
| `graceful-shutdown`            | ✗           | Tokio based graceful shutdown handler                             | [Graceful shutdown](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/graceful_shutdown.rs)            |
| `dlq`                          | ✗           | Dead Letter Queue implementation                                  | [Full implementation example](https://github.com/kpn-dsh/dsh-sdk-platform-rs/blob/main/dsh_sdk/examples/dlq_implementation.rs) |

### Selecting Features

To pick only the features you need, disable the default features and enable specific ones. For instance, if you only want the Management API Token Fetcher:

```toml
[dependencies]
dsh_sdk = { version = , default-features = false, features = ["management-api-token-fetcher"] }
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
- An end-to-end setup of a DSH service using Kafka


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

---

## Migration Guide

If you are migrating from `v0.4.X`, please see the [migration guide](https://github.com/kpn-dsh/dsh-sdk-platform-rs/wiki/Migration-guide-(v0.4.X-%E2%80%90--v0.5.X)) for details on breaking changes and how to update your code accordingly.

