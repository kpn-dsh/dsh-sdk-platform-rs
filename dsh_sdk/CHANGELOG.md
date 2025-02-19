# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.1] - 2025-02-19

### Fixed
- Realm for Prod platform now has correct value

## [0.6.0] - 2025-01-22

### Changed
- Documentation refering to deprecated API

### Removed
- Removed deprecated API

## [0.5.0] - 2025-01-22
### Added
- DSH Kafka Config trait to configure kafka client with RDKafka implementation
- DSH Schema store API Client
- New public functions `dsh_sdk::certificates::Cert`
  - Bootstrap to DSH
  - Read certificates from PKI_CONFIG_DIR
  - Add support reading private key in DER format when reading from PKI_CONFIG_DIR
- Implement `TryFrom<&Str>` and `RryFrom<String>` for `dsh_sdk::Platform`
- AccessToken is now public in managemant api token fetcher

### Changed
- **Breaking change:** `DshError` is now split into error enums per feature flag to untangle mess
  - `dsh_sdk::DshError` only applies on `bootstrap` feature flag
- **Breaking change:** `dsh_sdk::Dsh::reqwest_client_config` now returns `reqwest::ClientConfig` instead of `Result<reqwest::ClientConfig>` 
- **Breaking change:** `dsh_sdk::Dsh::reqwest_blocking_client_config` now returns `reqwest::ClientConfig` instead of `Result<reqwest::ClientConfig>` 
- **Breaking change:** `dsh_sdk::utils::Dlq` does not require `Dsh`/`Properties` as argument anymore
- **Breaking change:** `dsh_sdk::utils::Dlq::new` is removed and replaced with `dsh_sdk::utils::Dlq::start` which starts the DLQ and returns a channel to send dlq messages
- **Breaking change:** Deprecated `dsh_sdk::dsh::properties` module
- **Breaking change:** Moved `dsh_sdk::rest_api_token_fetcher` to `dsh_sdk::management_api::token_fetcher` and renamed `RestApiTokenFetcher` to `ManagementApiTokenFetcher`
- **Breaking change:** `dsh_sdk::error::DshRestTokenError` renamed to `dsh_sdk::management_api::error::ManagementApiTokenError`
  - **NOTE** Cargo.toml feature flag `rest-token-fetcher` renamed to`management-api-token-fetcher` 
- Moved `dsh_sdk::dsh::datastream` to `dsh_sdk::datastream`
- Moved `dsh_sdk::dsh::certificates` to `dsh_sdk::certificates`
  - Private module `dsh_sdk::dsh::bootstrap` and `dsh_sdk::dsh::pki_config_dir` are now part of `certificates` module
- **Breaking change:** Moved `dsh_sdk::mqtt_token_fetcher` to `dsh_sdk::protocol_adapters::token` and renamed to `ApiClientTokenFetcher`
  - **NOTE** The code is refactored to follow the partial mediation and full mediation pattern
  - **NOTE** Cargo.toml feature flag `mqtt-token-fetcher`  renamed to `protocol-token`
- **Breaking change:** Renamed  `dsh_sdk::Platform` methods to more meaningful names
- **Breaking change:** Moved `dsh_sdk::dlq` to `dsh_sdk::utils::dlq` 
- **Breaking change:** Moved `dsh_sdk::graceful_shutdown` to `dsh_sdk::utils::graceful_shutdown`
- **Breaking change:** Moved `dsh_sdk::metrics` to `dsh_sdk::utils::metrics`
- **Breaking change:** `dsh_sdk::utils::metrics::start_metrics_server` requires `fn() -> String` which gathers and encodes metrics

### Removed
- Removed `dsh_sdk::rdkafka` public re-export, import `rdkafka` directly
  - **NOTE** Feature-flag `rdkafka-ssl` and `rdkafka-ssl-vendored` are removed!
- Removed re-export of `prometheus` and `lazy_static` in `metrics` module, if needed import them directly
  - **NOTE** See [examples](./examples/expose_metrics.rs) how to use the http server

- Removed `Default` trait for `Dsh` (original `Properties`) struct as this should not be public

### Fixed

## [0.4.11] -2024-09-30
### Fixed
- Retry mechanism for when PKI endpoint is not yet avaialble during rolling restart DSH ([#101](https://github.com/kpn-dsh/dsh-sdk-platform-rs/issues/101))

## [0.4.10] -2024-09-30
### Added
- Add new with client methods to REST and MQTTT token fetcher

### Changed
- used async aware Mutex in MQTTT token fetcher
- Cargo clippy suggestions


## [0.4.9] - 2024-09-27
### Changed
- Bugfix deadlock when running on mac together with Kafka Proxy
- Adjust logging to INFO for fetching environment variables

## [0.4.8] - 2024-09-13
### Fixed
- Correct Prod-lz endpoint 

## [0.4.7] - yanked - 2024-09-13
### Changed
- updated rest token endpoint due to CP migration

## [0.4.6] - 2024-08-24
### Added
- Add token fetcher for DSH MQTT
  - fetch token for MQTT
  - cache tokens till expiration time
  - support fetching tokens for multiple clients
### Changed
- Updated dependencies
  - Lazy_static to 1.5


## [0.4.5] - 2024-07-24
### Added
- Add additional/optional config for producers and consumer
  - The config can be set via environment variables

## [0.4.4] - 2024-07-15 
### Added
- Provide new methods on `Properties`
  - fetch_datastream: Highlevel async method to retun a `Result<Datastream>`
  - fetch_datastream_blocking: Highlevel blocking method to return a `Result<Datastream>`
  - reqwest_blocking_client_config: Returns a `Result<reqwest::blocking::ClientConfig>`
- Provide new methods on `Datastream`
  - fetch: Lowlevel async method to retun a `Result<Datastream>`
  - fetch_datastream: Lowlevel  blocking method to return a `Result<Datastream>`
- Add derive Debug to graceful_shutdown::Shutdown
- Provide optional way to give path to load local_datastreams.json (`LOCAL_DATASTREAMS_JSON`)

### Changed
- Verify if local_datastreams.json parses correctly, else panic instead of using datastreams default
- Optimized bootstrap sequence and moved fetching datastreams to a separate method in `Datastream`


## [0.4.3] - 2024-07-08
### Added
- Add missing get functions to datastreams struct

### Fixed
- Issue with building docsrs

## [0.4.2] - 2024-07-01
### Added
- Add token fetcher for DSH REST API
- Add platform enum with metadata
- Add example on how to use the token fetcher and rest api client


## [0.4.1] - 2024-06-13
### Added
- Add loading Certificates and Keys from $PKI_CONFIG_DIR
  - Compatbile with DSH VPN
  - Compatbile with Kafka Proxy
- Overwrrite tenant name via $DSH_TENANT_NAME variable

### Changed
- Restructure of the private functions to make it more modular
- Improved logging
- Improved API Documentation

## [0.4.0] - 2024-04-25

### Fixed
- Fixed vulnerability RUSTSEC-2023-0071 by replacing Picky with RCGen
- Fixed compile issues when defaullt feature = false and graceful_shutdown, metrics or DLQ is enabled

### Added
- Add Default implementation for Dsh::Properties
  - Points to localhost:9092 for kafka, localhost:8081 for schemastore
  - local_datastreams.json is now optional as it falssback to default values
- Overwrite Kafka config via environment variables for producer and consumer
- Add extra check in github actions to check for compile issues for all features independently

### Changed
- **Breaking change:** consumer_rdkafka_config and producer_rdkafka_config returns `ClientConfig` instead of `Result<ClientConfig>`
- **Breaking change:** certificates and keys are now returned as `T` instead of `Result<T>` 
- **Breaking change:** Private key is based on ECDSA instead of RSA 
- **Breaking change:** Error enum is now non_exhaustive

### Removed
- Removed return of Picky key struct is removed

## [0.3.1] - 2024-03-25
  
### Fixed
- Add missing prometheus export back to the SDK

## [0.3.0] - yanked - 2024-03-25
  
### Changed
- Metrics http server based on Hyper instead of Warp
  - **Breaking change:**  start_metrics_server() now returns a tokio::JoinHandle<Result<()>>
  - Runs per default on a separate thread
- Fixed documentation for dsh::Properties::get()

### Removed
- Deprecated Dsh::Properties::new() removed
- Removed unused dependencies in Cargo.toml

## [0.2.0] - 2024-03-06
  
### Added

- Make dsh::Properties lazily available dsh::Properties::get() 
- New low level certificate functions to the SDK
  - Return keys and certificates as DER format
  - Create client.key, client.crt and ca.crt in a folder
- Write datastreams.json as file to a folder
- Read DSH_SECRET_TOKEN_PATH from environment and read secret token from file as fallback for DSH_SECRET_TOKEN
  - Required for running in system space
- Logging when reading required env variables
- Task_id to Properties
- Some missing unit tests

### Changed
Most breaking changes are related to the new low level API and do not impact normal use of the SDK.
- Add deprecation warning to dsh::Properties::new() to use dsh::Properties::get() instead
- **Breaking change:**  Return producer config in a `Result<ClientConfig>` instead of a `ClientConfig`
- **Breaking change:**  Pem formatted certificates and keys returns a `Result<String>` instead of a string
- **Breaking change:**  Return certificates in a `Result<Cert>` instead of a `Option<Cert>`
- **Breaking change:**  Return borrowed references from Datastream struct instead of owned values.
- Fix dsh::Properties::new() when feature 'local' is disabled
- Make initialization blocking
- Improved logging

### Removed
- Removed unused dependency in Cargo.toml
  

## [0.1.3] - 2024-02-23

### Added
- Added blocking function to initiate Properties

## [0.1.2] - 2024-02-12

### Added
- Option to deploy example DSH service directly from makefile
- Add CHANGELOG.md

### Changed
- Improved logging in SDK with use of log crate
- Small bug fix in selecting group_id
- Improved API Documentation in SDK
- Improved example DSH service (show)
- Updated [CONTRIBUTING.md](CONTRIBUTING.md)

## [0.1.1] - 2024-01-25

### Changed
- Updated READNE.md and SECURITY.md
- Set a more stricter version of dependencies of Tokio and Warp to avoid vulnerabilities

### Removed
- Removed ignore RUSTSEC-2023-0071 in cargo audit in CI pipeline (Marvin attack)
  - Vulenerability is not relevant for the SDK


## [0.1.0] - 2024-01-25

### Added

- Added DSH_SDK crate
- Added examples on how to use SDK
- Added a DSH service example
