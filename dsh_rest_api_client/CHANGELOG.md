# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2025-07-25
### Changed
- updated crate to OpenAPI spec: 1.10.0
- **breaking change:** `third_party_bucket_get_by_tenant_thirdpartybucketconcession_by_id_configuration` returns now a `ThirdPartyBucketConcessionConfiguration`
    - This was initially misconfigured in the OpenAPI spec
- Progenitor version: 0.10.0


## [0.4.0] - 2025-05-28
### Changed
- updated crate to OpenAPI spec: 1.9.2
    - Based on progenitor 0.10.0 instead of 0.9.0
- Progenitor client added new traits for client
- Adding API version to headers

## [0.3.0] - 2025-01-06
### Added
- endpoints to manage tenant and child tenant and Kafka ACLs

### Changed
- updated crate to OpenAPI spec: 1.9.0
    - Possible breaking changes
    - Based on progenitor 0.9.0 instead of 0.7.0

## [0.2.0] - 2024-09-04

### Added
- endpoints to manage public/internal streams

### Changed
- updated crate to OpenAPI spec: 1.8.0
    - **Breaking change:** Enum KafkaProxyZone (Internal renamed to Private)
    - **Breaking change:** robot_get_robot_by_tenant_generate_secret -> robot_post_robot_by_tenant_generate_secret


## [0.1.0] - 2024-07-01

### Added

- Added dsh_rest_api_client crate based on 1.7.0 OpenAPI spec
- Added update_openapi_spec.py script to add missing fields to the OpenAPI spec
