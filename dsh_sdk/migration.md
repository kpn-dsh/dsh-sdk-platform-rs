# Index
- [Why restructuring?](#why-restructuring)
    - [Maintainability](#maintainability)
    - [Breaking changes due to re-exports](#breaking-changes-due-to-re-exports)
- [Expected Changes and Impact](#expected-changes-and-impact)
    - [Current](#current)
    - [Expected](#expected)
    - [Impact](#impact)
        - [Current SDK usage](#current-sdk-usage)
        - [Expected SDK usage](#expected-sdk-usage)
- [Migration process](#migration-process)
    - [v0.5.0 (expected release date: 2024-01-01)](#v050-expected-release-date-2024-01-01)
    - [v0.6.0 (expected release date: 2024-02-01)](#v060-expected-release-date-2024-02-01)

## Why restructuring?
The SDK was initially built around the Kafka protocol and easily connect to DSH. Meanwhile, we also want to support other protocols (e.g. MQTT and Http) and add extra utilities to the SDK to make it more easier for developer to work with DSH. 

### Maintainability
To support the other protocols and new features, the current structure of the API of the SDK is not ideal to extend, as the API currently is focussed on RDKafka. To add the other features we noticed it will be fairly hard to maintain the codebase.

### Breaking changes due to re-exports
Also, some external libraries are re-exported, we would like to minimize these dependencies to avoid breaking changes. For example RDKafka is fully re-exported and recently RDKakfka released a major version, which will be technically a breaking change for the SDK. The idea is to move away from the `re-exports` and the DSH_SDK will implement a Trait to extend RDKakfka client configuration to create. See [Expected Changes and Impact](#expected-changes-and-impact)

## Expected Changes and Impact
These expected changes will have impact on your code as well. Below is a high-level overview of the current and expected API structure.

### Current

```rust
crate dsh_sdk
├── mod dlq: pub
├── mod dsh: pub
│   ├── mod bootstrap: priv
│   ├── mod certificates: pub
│   ├── mod datastream: pub
│   ├── mod pki_config_dir: priv
│   └── mod properties: pub
├── mod error: pub
├── mod graceful_shutdown: pub
├── mod metrics: pub
├── mod mqtt_token_fetcher: pub
├── mod rest_api_token_fetcher: pub
└── mod utils: pub
```

### Expected

```rust
crate dsh_sdk
├── mod certificates: pub
│   ├── mod bootstrap: priv
│   └── mod pki_config_dir: priv
├── mod datastream: pub
├── mod properties: pub 
├── mod error: pub (maybe create sepeate error for each module, to avoid mess with cfg feautres flags)
├── mod protocol_adapters: pub
|   ├── mod kafka_protocol: pub (replaces get_rdkafka_consumer_config/get_rdkafka_producer_config from properties struct, and  will be a trait that extends the RDKafka config)
│   ├── mod mqtt_protocol: pub (new, to be build)
│   ├── mod http_protocol: pub (new, to be build)
│   └── mod token_fetcher: pub (renamed from mqtt_token_fetcher)
├── mod deserializers
│   └── mod dsh_envelope
├── mod management_api: pub
|   └── mod token_fetcher: pub (renamed from rest_api_token_fetcher)
├── mod schema_store_api
└── mod utils: pub 
    ├── mod dlq: pub
    ├── mod graceful_shutdown: pub
    └── mod metrics: pub
```

### Impact
The biggest impact will be on how currently RDKafka is used. In the new situation you will have to import RDKafka and/or Prometheus crate your self. The SDK will provide a trait to extend the RDKafka client configuration. 

#### Current SDK usage
In v0.4.X the SDK would have been used something like this:
```rust
use dsh_sdk::Properties;
use dsh_sdk::rdkafka::consumer::{Consumer, StreamConsumer};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let dsh_properties = Properties::get();
    let consumer: StreamConsumer = dsh_properties.consumer_rdkafka_config().create()?;
    Ok(())
}
```

#### Expected SDK usage
In v0.5.X / v0.6.X the SDK should be used something like this:
```rust
use dsh_sdk::DshKafkaConfig; // Import the trait that extends the RDKafka ClientConfig
use rdkafka::ClientConfig; // Import RDKafka directly
use rdkafka::consumer::{Consumer, StreamConsumer}; // Import RDKafka directly

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let consumer: StreamConsumer =     let consumer: StreamConsumer = ClientConfig::new()
        .dsh_config() // Comes from the DshKafkaConfig trait
        .create();?;
    Ok(())
}
```

## Migration process
The migration process will be done in multiple steps. V0.5.0 will be firt made available as a Release Candidate to test the new API in some of the UniBox products. Here we want to check if the new API is easy to use and to gather some feedback.

### v0.5.0 (expected release date: 2025-01-01)
v0.5.0 will be the first release with the new structure parallel to the old structure. 
- Old API will be aliases to the new API, so YOUR current code is still valid.
- Old API will have deprecated warnings with explaination how to migrate to the new API
- Re-exports of RDkafka are still available, but will have a deprecation warning
- Additional features will be added (e.g. MQTT and HTTP protocol adapters)

### v0.6.0 (expected release date: 2025-02-01)
v0.6.0 will be the first release with the old API removed.
- Old API will be removed
- Re-exports will be removed
    
