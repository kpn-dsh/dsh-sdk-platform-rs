# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2024-03-04
  
### Added

- Added low level certificate functions to the SDK
  - Return keys and certificates as DER format
  - Create client.key, client.crt and ca.crt in a folder
- Added logging when reading required env variables

### Changed
- **Breaking change:**  Pem formatted certificates and keys returns a Result< String > instead of a string
- **Breaking change:**  Return certificates in a Result < Cert > instead of a Option< Cert >
- **Breaking change:**  Return producer config in a Result < ClientConfig > instead of a ClientConfig
- Fix dsh::Properties::new() when feature 'local' is disabled

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
