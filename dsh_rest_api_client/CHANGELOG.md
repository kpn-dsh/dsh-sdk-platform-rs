# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
 - Breaking change: Enum KafkaProxyZone (Internal renamed to Private)
 - Breaking change: robot_get_robot_by_tenant_generate_secret -> robot_post_robot_by_tenant_generate_secret


## [0.1.0] - 2024-07-01

### Added

- Added dsh_rest_api_client crate based on 1.7.0 OpenAPI spec
- Added update_openapi_spec.py script to add missing fields to the OpenAPI spec
