# dsh-sdk-platform-rust
Rust SDK for DSH

## Feautures
Bootstrap to DSH an some common functions. This library is intended to be used as a base for services that will be used to interact with DSH. It is not intended to be used directly.

| **feature** | **default** | **Description** |
|---|---|---|
| `local` | &check; | Use bootstrap in a local environment* |
| `graceful_shutdown` | &check; | Create a signal handler for implementing a graceful shutdown |
| `dlq` | &cross; | Dead Letter Queue implementation (experimental) |
| `rdkafka-ssl` | &check; | Dynamically link to librdkafka to a locally installed OpenSSL |
| `rdkafka-ssl-vendored` | &cross; | Build OpenSSL during compile and statically link librdkafka<br>(No initial install required in environment, does compile time) |

See api documentation for more information on how to use these features including.

\* Requires a local_datastream.json in your project root.

## Usage
This library is dependent on CMAKE to use rdkafka. Make sure it is installed in your environment and/or Dockerfile where you are compiling.

For example, in your Dockerfile:
```dockerfile
RUN apt-get update && apt-get install -y \
    cmake 
```
