# DSH Rest API Client
An OpenAPI Spec implementation for the DSH REST API.

## Description
This crate is part of the [DSH_SDK](https://crates.io/crates/dsh_sdk). It provides a Rust client for the DSH REST API. This client is generated from the OpenAPI specification using [Progenitor](https://github.com/oxidecomputer/progenitor).

### Versions
The versions of this crate are tied to the versions of the OpenAPI spec and the Progenitor version used to generate the client.

#### DSH Rest API Client: 0.4,0
- OpenAPI spec: 1.9.2
- Progenitor version: 0.10.0

#### DSH Rest API Client: 0.3.0
- OpenAPI spec: 1.9.0
- Progenitor version: 0.9.0

#### DSH Rest API Client: 0.2.0
- OpenAPI spec: 1.8.0
- Progenitor version: 0.7.0

#### DSH Rest API Client: 0.1.0
- OpenAPI spec: 1.7.0
- Progenitor version: 0.7.0

### Goals
This crate provides:
- A client with all methods to call all DSH API endpoints
- Pure code generation from the OpenAPI spec

### Non-Goals
This crate does not provide:
- Authentication or authorization to DSH
- Token management
- Functionality to select specific platform/base URL

These goals are provided by the [DSH_SDK](https://crates.io/crates/dsh_sdk) crate.

## Recomended usage
It is recommended to use the Rest Token Fetcher from the `dsh_sdk` crate. To do this, add the following to your Cargo.toml file:

```toml
[dependencies]
dsh_rest_api_client = "0.4,0"
dsh_sdk = { version = "0.6", features = ["management-api-token-fetcher"], default-features = false }
tokio = { version = "1", features = ["full"] }
```

To use the client in your project:
```rust
use dsh_rest_api_client::Client;
use dsh_sdk::{Platform, ManagementApiTokenFetcherBuilder};

const CLIENT_SECRET: &str = "YOUR_API_KEY_HERE";
const TENANT: &str = "tenant-name";

#[tokio::main]
async fn main() {
    let platform = Platform::NpLz;
    let client = Client::new(platform.endpoint_rest_api());

    let tf = ManagementApiTokenFetcherBuilder::new(platform)
        .tenant_name(TENANT.to_string())
        .client_secret(CLIENT_SECRET.to_string())
        .build()
        .unwrap();

    let response = client
        .topic_get_by_tenant_topic(TENANT, &tf.get_token().await.unwrap())
        .await;

    println!("Available topics: {:#?}", response);
}
```
## Changelog
See [CHANGELOG.md](CHANGELOG.md) for all changes per version.

## License
See [LICENSE](../LICENSE) for more information on the license for this project.

---
_Copyright (c) Koninklijke KPN N.V._ 