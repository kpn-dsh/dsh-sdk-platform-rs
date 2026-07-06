# How to upgrade
To upgrade to a newer version of the OpenAPI spec, we can regenerate the client using Progenitor.

## Install Rust  Nightly
The Rust formatter of Progenitor requires Rust nightly to run. Install Rust nightly using rustup.

```shell
rustup install nightly
```

## Install progenitor
install progenitor using cargo

```shell
cargo install cargo-progenitor --version 0.12.0
```

## Build lib
Put the new openapi spec json in `dsh_rest_api_client/openapi_spec` directory.

Then execute the following Make command with the correct filename. This will clean the openapi spec and generate the lib.rs with correct feature flags

```shell
make build SPEC=openapi_1_13_0.json
```