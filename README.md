# dsh-sdk-platform-rs

[![Build Status](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yml/badge.svg)](https://github.com/kpn-dsh/dsh-sdk-platform-rs/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/kpn-dsh/dsh-sdk-platform-rs)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Description
This library is a can be used to interact with the DSH Platform. It is intended to be used as a base for services that will be used to interact with DSH. It is not intended to be used directly. Features include:
- Connect to DSH 
- Fetch Kafka Properties (datastream)
- Common functions 
  - Preconfigured RDKafka client config
  - Preconfigured Reqwest client config (for schema store)
- Graceful shutdown
- Dead Letter Queue (experimental)

## Usage

To use this SDK with the default features in your project, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = "0.1.0"
```

However, if you would like to use only specific features, you can specify them in your Cargo.toml file. For example, if you would like to use only the bootstrap feature, add the following to your Cargo.toml file:
  
```toml
[dependencies]
dsh_sdk = { version = "0.1.0", default-features = false, features = ["bootstrap"] }
```

See [feature flags](#feature-flags) for more information on the available features.

To use this SDK in your project
```rust
use dsh_sdk::dsh::Properties;

#[tokio::main]
async fn main() {
    let dsh_properties = Properties::new().await.unwrap();
    // get a rdkafka consumer config for example
    let consumer_config = dsh_properties.consumer_rdkafka_config().create().unwrap();
}
```

## Api doc
See the [api documentation](https://docs.rs/dsh_sdk/latest/dsh_sdk/) for more information on how to use this library.

### Local development
Add a [local_datastream.json](local_datastream.json) to your project root.

### Note
Rdkafka and thereby this library is dependent on CMAKE. Make sure it is installed in your environment and/or Dockerfile where you are compiling.
See dockerfile in [example_dsh_service](/example_dsh_service/Dockerfile) for an example.

## Examples
See folder [examples](/examples/) for simple examples on how to use the SDK.

### Full service example
See folder [example_dsh_service](/example_dsh_service/) for a full service, including how to build the Rust project and post it to Harbor. See [readme](example_dsh_service/README.md) for more information.

## Feauture flags

The following features are available in this library and can be enabled/disabled in your Cargo.toml file.:

| **feature** | **default** | **Description** |
|---|---|---|
| `bootstrap` | &check; | Generate signed certificate and fetch datastreams info <br> Also makes certificates avaiable, to be used as lowlevel API |
| `local` | &check; | Use the SDK in your local environment* |
| `metrics` | &check; | Enable (custom) metrics for your service |
| `graceful_shutdown` | &check; | Create a signal handler for implementing a graceful shutdown |
| `dlq` | &cross; | Dead Letter Queue implementation (experimental) |
| `rdkafka-ssl` | &check; | Dynamically link to librdkafka to a locally installed OpenSSL |
| `rdkafka-ssl-vendored` | &cross; | Build OpenSSL during compile and statically link librdkafka <br> (No initial install required in environment, slower compile time) |

See api documentation for more information on how to use these features including.

\* Requires a [local_datastream.json](local_datastream.json) in your project root.


## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information on how to contribute to this project.

## License
See [LICENSE](LICENSE) for more information on the license for this project.

## Security
See [SECURITY.md](SECURITY.md) for more information on the security policy for this project.
