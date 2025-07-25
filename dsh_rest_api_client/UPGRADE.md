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
cargo install cargo-progenitor --version 0.10.0
```

## Prepare the OpenAPI spec file
Download the OpenAPI spec file from the DSH API documentation.

The spec of DSH is missing the required `OperationId` field and authentication. To add this, run the following python script:

```shell
python3 update_openapi_spec.py path/to/openapi.json
```

## Generate the client
To generate the client run the following command:

```shell
cargo +nightly  progenitor -i dsh_rest_api_client/openapi_spec/openapi_1_8_0.json -o tmp_dsh_rest_api_client -n dsh_rest_api_client --version 0.1.0 --include-client true
```

## Update the client
Copy the generated `lib.rs` from `tmp_dsh_rest_api_client` to the src folder of `dsh_rest_api_client` in the `dsh_sdk` repository and update the `README.md` file and `cargo.toml` with the new version number. 
