# DSH Rest API Client

This crate is part of the [DSH_SDK](https://crates.io/crates/dsh_sdk). It provides a Rust client for the DSH REST API. This client is generated from the OpenAPI specification using [Progenitor](https://github.com/oxidecomputer/progenitor).

Based on 
- OpenAPI spec: 1.7.0
- Progenitor version: 0.7.0

## Goals
This crate provides:
- A client with all methods to call all DSH API endpoints
- Pure code generation from the OpenAPI spec

## Non-Goals
This crate does not provide:
- Authentication or authorization to DSH
- Functionality to select specific platform/base URL

These goals are provided by the `dsh_sdk` crate.

## Recomended usage

It is recommended to use the `dsh_sdk` crate with the `rest_api_client` feature enabled. This will include the `dsh_rest_api_client` crate as a dependency. 

```toml
[dependencies]
dsh_sdk = { version = "0.4", features = ["rest_api_client"], default-features = false }
```
