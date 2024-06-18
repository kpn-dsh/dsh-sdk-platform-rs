mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///information on a certificate which is provisioned on the platform
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "information on a certificate which is provisioned on
    /// the platform",
    ///  "type": "object",
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Certificate"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ActualCertificate_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ActualCertificate {
        #[serde(rename = "certChainSecret")]
        pub cert_chain_secret: String,
        #[serde(rename = "distinguishedName")]
        pub distinguished_name: String,
        #[serde(rename = "dnsNames")]
        pub dns_names: Vec<String>,
        #[serde(rename = "keySecret")]
        pub key_secret: String,
        #[serde(rename = "notAfter")]
        pub not_after: chrono::DateTime<chrono::offset::Utc>,
        #[serde(rename = "notBefore")]
        pub not_before: chrono::DateTime<chrono::offset::Utc>,
        #[serde(
            rename = "passphraseSecret",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub passphrase_secret: Option<String>,
        #[serde(rename = "serialNumber")]
        pub serial_number: String,
    }

    impl From<&ActualCertificate> for ActualCertificate {
        fn from(value: &ActualCertificate) -> Self {
            value.clone()
        }
    }

    ///ActualCertificateAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "distinguishedName",
    ///    "dnsNames",
    ///    "notAfter",
    ///    "notBefore",
    ///    "serialNumber"
    ///  ],
    ///  "properties": {
    ///    "distinguishedName": {
    ///      "type": "string"
    ///    },
    ///    "dnsNames": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "notAfter": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "notBefore": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "serialNumber": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ActualCertificateAllOf {
        #[serde(rename = "distinguishedName")]
        pub distinguished_name: String,
        #[serde(rename = "dnsNames")]
        pub dns_names: Vec<String>,
        #[serde(rename = "notAfter")]
        pub not_after: chrono::DateTime<chrono::offset::Utc>,
        #[serde(rename = "notBefore")]
        pub not_before: chrono::DateTime<chrono::offset::Utc>,
        #[serde(rename = "serialNumber")]
        pub serial_number: String,
    }

    impl From<&ActualCertificateAllOf> for ActualCertificateAllOf {
        fn from(value: &ActualCertificateAllOf) -> Self {
            value.clone()
        }
    }

    ///AllocationStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "derivedFrom": "derivedFrom",
    ///      "notifications": [
    ///        {
    ///          "args": {
    ///            "key": "args"
    ///          },
    ///          "message": "message",
    ///          "remove": true
    ///        },
    ///        {
    ///          "args": {
    ///            "key": "args"
    ///          },
    ///          "message": "message",
    ///          "remove": true
    ///        }
    ///      ],
    ///      "provisioned": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "notifications",
    ///    "provisioned"
    ///  ],
    ///  "properties": {
    ///    "derivedFrom": {
    ///      "description": "pointer to the parent allocation or limit that
    /// caused this allocation to be implicitly created\n",
    ///      "type": "string"
    ///    },
    ///    "notifications": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Notification"
    ///      }
    ///    },
    ///    "provisioned": {
    ///      "description": "indicates whether configuration and actual state
    /// match",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocationStatus {
        ///pointer to the parent allocation or limit that caused this
        /// allocation to be implicitly created
        #[serde(
            rename = "derivedFrom",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub derived_from: Option<String>,
        pub notifications: Vec<Notification>,
        ///indicates whether configuration and actual state match
        pub provisioned: bool,
    }

    impl From<&AllocationStatus> for AllocationStatus {
        fn from(value: &AllocationStatus) -> Self {
            value.clone()
        }
    }

    ///AllocationStatus1
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "$ref": "#/components/schemas/AllocationStatus"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AllocationStatus1(pub AllocationStatus);
    impl std::ops::Deref for AllocationStatus1 {
        type Target = AllocationStatus;
        fn deref(&self) -> &AllocationStatus {
            &self.0
        }
    }

    impl From<AllocationStatus1> for AllocationStatus {
        fn from(value: AllocationStatus1) -> Self {
            value.0
        }
    }

    impl From<&AllocationStatus1> for AllocationStatus1 {
        fn from(value: &AllocationStatus1) -> Self {
            value.clone()
        }
    }

    impl From<AllocationStatus> for AllocationStatus1 {
        fn from(value: AllocationStatus) -> Self {
            Self(value)
        }
    }

    ///AppCatalogApp
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "configuration": "configuration",
    ///      "manifestUrn": "manifestUrn",
    ///      "name": "name",
    ///      "resources": {
    ///        "key": ""
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "manifestUrn",
    ///    "name",
    ///    "resources"
    ///  ],
    ///  "properties": {
    ///    "configuration": {
    ///      "type": "string"
    ///    },
    ///    "manifestUrn": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "resources": {
    ///      "description": "child resources",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "oneOf": [
    ///          {
    ///            "$ref": "#/components/schemas/Application"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Bucket"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Certificate"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Secret"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Topic"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Vhost"
    ///          },
    ///          {
    ///            "$ref": "#/components/schemas/Volume"
    ///          }
    ///        ]
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AppCatalogApp {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<String>,
        #[serde(rename = "manifestUrn")]
        pub manifest_urn: String,
        pub name: String,
        ///child resources
        pub resources: std::collections::HashMap<String, AppCatalogAppResourcesValue>,
    }

    impl From<&AppCatalogApp> for AppCatalogApp {
        fn from(value: &AppCatalogApp) -> Self {
            value.clone()
        }
    }

    ///AppCatalogAppConfiguration
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "configuration": {
    ///        "key": "configuration"
    ///      },
    ///      "manifestUrn": "manifestUrn",
    ///      "name": "name",
    ///      "stopped": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "configuration",
    ///    "manifestUrn",
    ///    "name",
    ///    "stopped"
    ///  ],
    ///  "properties": {
    ///    "configuration": {
    ///      "description": "configuration parameters to be used in AppCatalog
    /// manifest",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "manifestUrn": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "stopped": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AppCatalogAppConfiguration {
        ///configuration parameters to be used in AppCatalog manifest
        pub configuration: std::collections::HashMap<String, String>,
        #[serde(rename = "manifestUrn")]
        pub manifest_urn: String,
        pub name: String,
        pub stopped: bool,
    }

    impl From<&AppCatalogAppConfiguration> for AppCatalogAppConfiguration {
        fn from(value: &AppCatalogAppConfiguration) -> Self {
            value.clone()
        }
    }

    ///AppCatalogAppResourcesValue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/Application"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Bucket"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Certificate"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Secret"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Topic"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Vhost"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Volume"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum AppCatalogAppResourcesValue {
        Application(Application),
        Bucket(Bucket),
        Certificate(Certificate),
        Secret(Secret),
        Topic(Topic),
        Vhost(Vhost),
        Volume(Volume),
    }

    impl From<&AppCatalogAppResourcesValue> for AppCatalogAppResourcesValue {
        fn from(value: &AppCatalogAppResourcesValue) -> Self {
            value.clone()
        }
    }

    impl From<Application> for AppCatalogAppResourcesValue {
        fn from(value: Application) -> Self {
            Self::Application(value)
        }
    }

    impl From<Bucket> for AppCatalogAppResourcesValue {
        fn from(value: Bucket) -> Self {
            Self::Bucket(value)
        }
    }

    impl From<Certificate> for AppCatalogAppResourcesValue {
        fn from(value: Certificate) -> Self {
            Self::Certificate(value)
        }
    }

    impl From<Secret> for AppCatalogAppResourcesValue {
        fn from(value: Secret) -> Self {
            Self::Secret(value)
        }
    }

    impl From<Topic> for AppCatalogAppResourcesValue {
        fn from(value: Topic) -> Self {
            Self::Topic(value)
        }
    }

    impl From<Vhost> for AppCatalogAppResourcesValue {
        fn from(value: Vhost) -> Self {
            Self::Vhost(value)
        }
    }

    impl From<Volume> for AppCatalogAppResourcesValue {
        fn from(value: Volume) -> Self {
            Self::Volume(value)
        }
    }

    ///AppCatalogManifest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "draft": true,
    ///      "lastModified": 0.8008281904610115,
    ///      "payload": "payload"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "draft",
    ///    "lastModified",
    ///    "payload"
    ///  ],
    ///  "properties": {
    ///    "draft": {
    ///      "type": "boolean"
    ///    },
    ///    "lastModified": {
    ///      "description": "creation timestamp of the secret",
    ///      "type": "number"
    ///    },
    ///    "payload": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AppCatalogManifest {
        pub draft: bool,
        #[serde(rename = "lastModified")]
        pub last_modified: f64,
        pub payload: String,
    }

    impl From<&AppCatalogManifest> for AppCatalogManifest {
        fn from(value: &AppCatalogManifest) -> Self {
            value.clone()
        }
    }

    ///Application
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "cpus": 0.8008281904610115,
    ///      "env": {
    ///        "key": "env"
    ///      },
    ///      "exposedPorts": {
    ///        "key": {
    ///          "auth": "auth",
    ///          "mode": "mode",
    ///          "paths": [
    ///            {
    ///              "prefix": "prefix"
    ///            },
    ///            {
    ///              "prefix": "prefix"
    ///            }
    ///          ],
    ///          "serviceGroup": "serviceGroup",
    ///          "tls": "auto",
    ///          "vhost": "vhost",
    ///          "whitelist": "whitelist"
    ///        }
    ///      },
    ///      "healthCheck": {
    ///        "path": "/",
    ///        "port": 0,
    ///        "protocol": "http"
    ///      },
    ///      "image": "image",
    ///      "instances": 0,
    ///      "mem": 0,
    ///      "metrics": {
    ///        "path": "/metrics",
    ///        "port": 0
    ///      },
    ///      "needsToken": true,
    ///      "readableStreams": [
    ///        "readableStreams",
    ///        "readableStreams"
    ///      ],
    ///      "secrets": [
    ///        {
    ///          "injections": [
    ///            {
    ///              "key": "injections"
    ///            },
    ///            {
    ///              "key": "injections"
    ///            }
    ///          ],
    ///          "name": "name"
    ///        },
    ///        {
    ///          "injections": [
    ///            {
    ///              "key": "injections"
    ///            },
    ///            {
    ///              "key": "injections"
    ///            }
    ///          ],
    ///          "name": "name"
    ///        }
    ///      ],
    ///      "singleInstance": false,
    ///      "spreadGroup": "spreadGroup",
    ///      "topics": [
    ///        "topics",
    ///        "topics"
    ///      ],
    ///      "user": "user",
    ///      "volumes": {
    ///        "key": {
    ///          "name": "name"
    ///        }
    ///      },
    ///      "writableStreams": [
    ///        "writableStreams",
    ///        "writableStreams"
    ///      ]
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "cpus",
    ///    "image",
    ///    "mem",
    ///    "user"
    ///  ],
    ///  "properties": {
    ///    "cpus": {
    ///      "description": "How many CPUs this application needs (0.5 = 50% of
    /// 1 cpu)",
    ///      "type": "number"
    ///    },
    ///    "env": {
    ///      "description": "Environment variables",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "exposedPorts": {
    ///      "description": "Exposes ports of your application outside the
    /// platform",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/PortMapping"
    ///      }
    ///    },
    ///    "healthCheck": {
    ///      "$ref": "#/components/schemas/HealthCheck"
    ///    },
    ///    "image": {
    ///      "description": "The container image to launch",
    ///      "type": "string",
    ///      "format": "docker_repo/tag:version"
    ///    },
    ///    "instances": {
    ///      "description": "Number of instances that need to be spun up for
    /// this app",
    ///      "default": 1,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "mem": {
    ///      "description": "Amount of memory your application needs in MB",
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "metrics": {
    ///      "$ref": "#/components/schemas/Metrics"
    ///    },
    ///    "needsToken": {
    ///      "description": "If true, the platform will provision a secret token
    /// in the `DSH_SECRET_TOKEN` environment variable. This token can be
    /// exchanged for a client certificate that can be used for authentication
    /// to, amongst others, the Kafka brokers.\n",
    ///      "default": true,
    ///      "type": "boolean"
    ///    },
    ///    "readableStreams": {
    ///      "description": "names of streams to which the application needs
    /// read access.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "secrets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ApplicationSecret"
    ///      }
    ///    },
    ///    "singleInstance": {
    ///      "description": "If true, the platform will ensure that there is
    /// always at most one instance of this application running at the same
    /// time. This impacts restart and upgrade behavior: A single-instance
    /// application will be terminated before a replacement is started, whereas
    /// an application that is not single-instance will remain running until its
    /// replacement has started and reports healthy. **Note** Applications that
    /// define volumes are always implicitly treated as single-instance, even if
    /// this flag is not set.",
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "spreadGroup": {
    ///      "description": "The spread group - if any - to be used to ensure
    /// instances of one or more applications are not scheduled onto the same
    /// node.",
    ///      "type": "string"
    ///    },
    ///    "topics": {
    ///      "description": "names of scratch topics to which the application
    /// needs access.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "user": {
    ///      "description": "The userid:groupid combination used to start the
    /// application container.",
    ///      "type": "string",
    ///      "format": "userid:groupid"
    ///    },
    ///    "volumes": {
    ///      "description": "The volumes to be mounted in the container. The
    /// dictionary key is the mount point.",
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/Application_volumes"
    ///      }
    ///    },
    ///    "writableStreams": {
    ///      "description": "names of streams to which the application needs
    /// write access.",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Application {
        pub cpus: f64,
        ///Environment variables
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub env: std::collections::HashMap<String, String>,
        ///Exposes ports of your application outside the platform
        #[serde(
            rename = "exposedPorts",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub exposed_ports: std::collections::HashMap<String, PortMapping>,
        #[serde(
            rename = "healthCheck",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub health_check: Option<HealthCheck>,
        ///The container image to launch
        pub image: String,
        ///Number of instances that need to be spun up for this app
        #[serde(default = "defaults::default_u64::<u64, 1>")]
        pub instances: u64,
        ///Amount of memory your application needs in MB
        pub mem: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub metrics: Option<Metrics>,
        ///If true, the platform will provision a secret token in the
        /// `DSH_SECRET_TOKEN` environment variable. This token can be exchanged
        /// for a client certificate that can be used for authentication to,
        /// amongst others, the Kafka brokers.
        #[serde(rename = "needsToken", default = "defaults::default_bool::<true>")]
        pub needs_token: bool,
        ///names of streams to which the application needs read access.
        #[serde(
            rename = "readableStreams",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub readable_streams: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub secrets: Vec<ApplicationSecret>,
        ///If true, the platform will ensure that there is always at most one
        /// instance of this application running at the same time. This impacts
        /// restart and upgrade behavior: A single-instance application will be
        /// terminated before a replacement is started, whereas an application
        /// that is not single-instance will remain running until its
        /// replacement has started and reports healthy. **Note** Applications
        /// that define volumes are always implicitly treated as
        /// single-instance, even if this flag is not set.
        #[serde(rename = "singleInstance", default)]
        pub single_instance: bool,
        ///The spread group - if any - to be used to ensure instances of one or
        /// more applications are not scheduled onto the same node.
        #[serde(
            rename = "spreadGroup",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub spread_group: Option<String>,
        ///names of scratch topics to which the application needs access.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub topics: Vec<String>,
        ///The userid:groupid combination used to start the application
        /// container.
        pub user: String,
        ///The volumes to be mounted in the container. The dictionary key is
        /// the mount point.
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub volumes: std::collections::HashMap<String, ApplicationVolumes>,
        ///names of streams to which the application needs write access.
        #[serde(
            rename = "writableStreams",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub writable_streams: Vec<String>,
    }

    impl From<&Application> for Application {
        fn from(value: &Application) -> Self {
            value.clone()
        }
    }

    ///a secret to be injected as an environment variable in the application
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "a secret to be injected as an environment variable in
    /// the application",
    ///  "examples": [
    ///    {
    ///      "injections": [
    ///        {
    ///          "key": "injections"
    ///        },
    ///        {
    ///          "key": "injections"
    ///        }
    ///      ],
    ///      "name": "name"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "injections",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "injections": {
    ///      "description": "a list of environment variable names. The secret's
    /// value may be injected multiple times as different environment variables,
    /// so multiple environment variable names for the same secret can be
    /// provided",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "additionalProperties": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "name": {
    ///      "description": "the secret's name",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ApplicationSecret {
        ///a list of environment variable names. The secret's value may be
        /// injected multiple times as different environment variables, so
        /// multiple environment variable names for the same secret can be
        /// provided
        pub injections: Vec<std::collections::HashMap<String, String>>,
        ///the secret's name
        pub name: String,
    }

    impl From<&ApplicationSecret> for ApplicationSecret {
        fn from(value: &ApplicationSecret) -> Self {
            value.clone()
        }
    }

    ///ApplicationVolumes
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "name": "name"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "the full name of the volume that needs to be
    /// mounted in the container.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ApplicationVolumes {
        ///the full name of the volume that needs to be mounted in the
        /// container.
        pub name: String,
    }

    impl From<&ApplicationVolumes> for ApplicationVolumes {
        fn from(value: &ApplicationVolumes) -> Self {
            value.clone()
        }
    }

    ///BaseLimitValue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string",
    ///      "enum": [
    ///        "cpu",
    ///        "mem",
    ///        "certificateCount",
    ///        "secretCount",
    ///        "topicCount",
    ///        "partitionCount",
    ///        "consumerRate",
    ///        "producerRate",
    ///        "requestRate"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BaseLimitValue {
        pub name: BaseLimitValueName,
    }

    impl From<&BaseLimitValue> for BaseLimitValue {
        fn from(value: &BaseLimitValue) -> Self {
            value.clone()
        }
    }

    ///BaseLimitValueName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum BaseLimitValueName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&BaseLimitValueName> for BaseLimitValueName {
        fn from(value: &BaseLimitValueName) -> Self {
            value.clone()
        }
    }

    impl ToString for BaseLimitValueName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for BaseLimitValueName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for BaseLimitValueName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for BaseLimitValueName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for BaseLimitValueName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Bucket
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "encrypted": true,
    ///      "versioned": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "encrypted",
    ///    "versioned"
    ///  ],
    ///  "properties": {
    ///    "encrypted": {
    ///      "type": "boolean"
    ///    },
    ///    "versioned": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Bucket {
        pub encrypted: bool,
        pub versioned: bool,
    }

    impl From<&Bucket> for Bucket {
        fn from(value: &Bucket) -> Self {
            value.clone()
        }
    }

    ///BucketAccess
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "bucket": "bucket",
    ///      "credentialidentifierref": "credentialidentifierref",
    ///      "credentialsecretref": "credentialsecretref",
    ///      "name": "name",
    ///      "readable": true,
    ///      "writable": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "bucket",
    ///    "credentialidentifierref",
    ///    "credentialsecretref",
    ///    "name",
    ///    "readable",
    ///    "writable"
    ///  ],
    ///  "properties": {
    ///    "bucket": {
    ///      "type": "string"
    ///    },
    ///    "credentialidentifierref": {
    ///      "type": "string"
    ///    },
    ///    "credentialsecretref": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "readable": {
    ///      "type": "boolean"
    ///    },
    ///    "writable": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BucketAccess {
        pub bucket: String,
        pub credentialidentifierref: String,
        pub credentialsecretref: String,
        pub name: String,
        pub readable: bool,
        pub writable: bool,
    }

    impl From<&BucketAccess> for BucketAccess {
        fn from(value: &BucketAccess) -> Self {
            value.clone()
        }
    }

    ///BucketAccessConfiguration
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "bucket": "bucket",
    ///      "name": "name",
    ///      "readable": true,
    ///      "writable": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "bucket",
    ///    "name",
    ///    "readable",
    ///    "writable"
    ///  ],
    ///  "properties": {
    ///    "bucket": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "readable": {
    ///      "type": "boolean"
    ///    },
    ///    "writable": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BucketAccessConfiguration {
        pub bucket: String,
        pub name: String,
        pub readable: bool,
        pub writable: bool,
    }

    impl From<&BucketAccessConfiguration> for BucketAccessConfiguration {
        fn from(value: &BucketAccessConfiguration) -> Self {
            value.clone()
        }
    }

    ///BucketAccessStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "bucket": "bucket",
    ///        "credentialidentifierref": "credentialidentifierref",
    ///        "credentialsecretref": "credentialsecretref",
    ///        "name": "name",
    ///        "readable": true,
    ///        "writable": true
    ///      },
    ///      "configuration": {
    ///        "bucket": "bucket",
    ///        "name": "name",
    ///        "readable": true,
    ///        "writable": true
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/BucketAccess"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/BucketAccessConfiguration"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BucketAccessStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<BucketAccess>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<BucketAccessConfiguration>,
        pub status: AllocationStatus,
    }

    impl From<&BucketAccessStatus> for BucketAccessStatus {
        fn from(value: &BucketAccessStatus) -> Self {
            value.clone()
        }
    }

    ///BucketStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "encrypted": true,
    ///        "versioned": true
    ///      },
    ///      "configuration": {
    ///        "encrypted": true,
    ///        "versioned": true
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/Bucket"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/Bucket"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BucketStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<Bucket>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Bucket>,
        pub status: AllocationStatus,
    }

    impl From<&BucketStatus> for BucketStatus {
        fn from(value: &BucketStatus) -> Self {
            value.clone()
        }
    }

    ///BucketWatch
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "bucket": "bucket"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "bucket"
    ///  ],
    ///  "properties": {
    ///    "bucket": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BucketWatch {
        pub bucket: String,
    }

    impl From<&BucketWatch> for BucketWatch {
        fn from(value: &BucketWatch) -> Self {
            value.clone()
        }
    }

    ///BucketWatchStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "bucket": "bucket"
    ///      },
    ///      "configuration": {
    ///        "bucket": "bucket"
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/BucketWatch"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/BucketWatch"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BucketWatchStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<BucketWatch>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<BucketWatch>,
        pub status: AllocationStatus,
    }

    impl From<&BucketWatchStatus> for BucketWatchStatus {
        fn from(value: &BucketWatchStatus) -> Self {
            value.clone()
        }
    }

    ///information on a certificate which is wanted on the platform but may not
    /// yet be provisioned
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "information on a certificate which is wanted on the
    /// platform but may not yet be provisioned",
    ///  "examples": [
    ///    {
    ///      "certChainSecret": "certChainSecret",
    ///      "keySecret": "keySecret",
    ///      "passphraseSecret": "passphraseSecret"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "certChainSecret",
    ///    "keySecret"
    ///  ],
    ///  "properties": {
    ///    "certChainSecret": {
    ///      "type": "string"
    ///    },
    ///    "keySecret": {
    ///      "type": "string"
    ///    },
    ///    "passphraseSecret": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Certificate {
        #[serde(rename = "certChainSecret")]
        pub cert_chain_secret: String,
        #[serde(rename = "keySecret")]
        pub key_secret: String,
        #[serde(
            rename = "passphraseSecret",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub passphrase_secret: Option<String>,
    }

    impl From<&Certificate> for Certificate {
        fn from(value: &Certificate) -> Self {
            value.clone()
        }
    }

    ///CertificateStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": null,
    ///      "configuration": {
    ///        "certChainSecret": "certChainSecret",
    ///        "keySecret": "keySecret",
    ///        "passphraseSecret": "passphraseSecret"
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/ActualCertificate"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/Certificate"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CertificateStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<ActualCertificate>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Certificate>,
        pub status: AllocationStatus,
    }

    impl From<&CertificateStatus> for CertificateStatus {
        fn from(value: &CertificateStatus) -> Self {
            value.clone()
        }
    }

    ///ChildList
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "array",
    ///  "items": {
    ///    "type": "string"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ChildList(pub Vec<String>);
    impl std::ops::Deref for ChildList {
        type Target = Vec<String>;
        fn deref(&self) -> &Vec<String> {
            &self.0
        }
    }

    impl From<ChildList> for Vec<String> {
        fn from(value: ChildList) -> Self {
            value.0
        }
    }

    impl From<&ChildList> for ChildList {
        fn from(value: &ChildList) -> Self {
            value.clone()
        }
    }

    impl From<Vec<String>> for ChildList {
        fn from(value: Vec<String>) -> Self {
            Self(value)
        }
    }

    ///ClientSecret
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "createdDate": 0.8008281904610115,
    ///      "value": "value"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "createdDate": {
    ///      "description": "creation timestamp of the secret",
    ///      "type": "number"
    ///    },
    ///    "value": {
    ///      "description": "the secret value",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientSecret {
        #[serde(
            rename = "createdDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub created_date: Option<f64>,
        ///the secret value
        pub value: String,
    }

    impl From<&ClientSecret> for ClientSecret {
        fn from(value: &ClientSecret) -> Self {
            value.clone()
        }
    }

    ///DataCatalogAsset
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "kind": "kind",
    ///      "name": "name"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "kind",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "kind": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DataCatalogAsset {
        pub kind: String,
        pub name: String,
    }

    impl From<&DataCatalogAsset> for DataCatalogAsset {
        fn from(value: &DataCatalogAsset) -> Self {
            value.clone()
        }
    }

    ///DataCatalogAssetStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "kind": "kind",
    ///        "name": "name"
    ///      },
    ///      "configuration": {
    ///        "kind": "kind",
    ///        "name": "name"
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/DataCatalogAsset"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/DataCatalogAsset"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DataCatalogAssetStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<DataCatalogAsset>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<DataCatalogAsset>,
        pub status: AllocationStatus,
    }

    impl From<&DataCatalogAssetStatus> for DataCatalogAssetStatus {
        fn from(value: &DataCatalogAssetStatus) -> Self {
            value.clone()
        }
    }

    ///Database
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "cpus": 1.0,
    ///      "extensions": [
    ///        "postgis",
    ///        "postgres_fdw",
    ///        "uuid-ossp"
    ///      ],
    ///      "instances": 3,
    ///      "mem": 3072,
    ///      "snapshotInterval": 3600,
    ///      "version": "2.11.1.0-8",
    ///      "volumeSize": 10
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "cpus",
    ///    "instances",
    ///    "mem",
    ///    "volumeSize"
    ///  ],
    ///  "properties": {
    ///    "cpus": {
    ///      "examples": [
    ///        1.0
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.5
    ///    },
    ///    "extensions": {
    ///      "examples": [
    ///        [
    ///          "postgis",
    ///          "postgres_fdw",
    ///          "uuid-ossp"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "instances": {
    ///      "examples": [
    ///        3
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 3.0
    ///    },
    ///    "mem": {
    ///      "examples": [
    ///        3072
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 2048.0
    ///    },
    ///    "snapshotInterval": {
    ///      "examples": [
    ///        3600
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 3600.0
    ///    },
    ///    "version": {
    ///      "examples": [
    ///        "2.11.1.0-8"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "volumeSize": {
    ///      "examples": [
    ///        10
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 10.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Database {
        pub cpus: f64,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub extensions: Vec<String>,
        pub instances: i64,
        pub mem: i64,
        #[serde(
            rename = "snapshotInterval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub snapshot_interval: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(rename = "volumeSize")]
        pub volume_size: i64,
    }

    impl From<&Database> for Database {
        fn from(value: &Database) -> Self {
            value.clone()
        }
    }

    ///DatabaseStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "cpus": 1.0,
    ///        "extensions": [
    ///          "postgis",
    ///          "postgres_fdw",
    ///          "uuid-ossp"
    ///        ],
    ///        "instances": 3,
    ///        "mem": 3072,
    ///        "snapshotInterval": 3600,
    ///        "version": "2.11.1.0-8",
    ///        "volumeSize": 10
    ///      },
    ///      "configuration": {
    ///        "cpus": 1.0,
    ///        "extensions": [
    ///          "postgis",
    ///          "postgres_fdw",
    ///          "uuid-ossp"
    ///        ],
    ///        "instances": 3,
    ///        "mem": 3072,
    ///        "snapshotInterval": 3600,
    ///        "version": "2.11.1.0-8",
    ///        "volumeSize": 10
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/Database"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/Database"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DatabaseStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<Database>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Database>,
        pub status: AllocationStatus,
    }

    impl From<&DatabaseStatus> for DatabaseStatus {
        fn from(value: &DatabaseStatus) -> Self {
            value.clone()
        }
    }

    ///Empty
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Empty {}
    impl From<&Empty> for Empty {
        fn from(value: &Empty) -> Self {
            value.clone()
        }
    }

    ///FlinkCluster
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "jobManager": {
    ///        "cpus": 0.3,
    ///        "mem": 1024
    ///      },
    ///      "taskManager": {
    ///        "cpus": 0.3,
    ///        "instances": 2,
    ///        "mem": 3072
    ///      },
    ///      "version": "version",
    ///      "zone": "zone"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "version",
    ///    "zone"
    ///  ],
    ///  "properties": {
    ///    "jobManager": {
    ///      "$ref": "#/components/schemas/FlinkJobManager"
    ///    },
    ///    "taskManager": {
    ///      "$ref": "#/components/schemas/FlinkTaskManager"
    ///    },
    ///    "version": {
    ///      "description": "Flink version",
    ///      "type": "string"
    ///    },
    ///    "zone": {
    ///      "description": "Network zone this cluster needs to run in.
    /// /components/schemas/Zone contains a list of available network zones in
    /// this platform.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlinkCluster {
        #[serde(
            rename = "jobManager",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub job_manager: Option<FlinkJobManager>,
        #[serde(
            rename = "taskManager",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub task_manager: Option<FlinkTaskManager>,
        ///Flink version
        pub version: String,
        ///Network zone this cluster needs to run in. /components/schemas/Zone
        /// contains a list of available network zones in this platform.
        pub zone: String,
    }

    impl From<&FlinkCluster> for FlinkCluster {
        fn from(value: &FlinkCluster) -> Self {
            value.clone()
        }
    }

    ///FlinkClusterStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "jobManager": {
    ///          "cpus": 0.3,
    ///          "mem": 1024
    ///        },
    ///        "taskManager": {
    ///          "cpus": 0.3,
    ///          "instances": 2,
    ///          "mem": 3072
    ///        },
    ///        "version": "version",
    ///        "zone": "zone"
    ///      },
    ///      "configuration": {
    ///        "jobManager": {
    ///          "cpus": 0.3,
    ///          "mem": 1024
    ///        },
    ///        "taskManager": {
    ///          "cpus": 0.3,
    ///          "instances": 2,
    ///          "mem": 3072
    ///        },
    ///        "version": "version",
    ///        "zone": "zone"
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/FlinkCluster"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/FlinkCluster"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlinkClusterStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<FlinkCluster>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<FlinkCluster>,
        pub status: AllocationStatus,
    }

    impl From<&FlinkClusterStatus> for FlinkClusterStatus {
        fn from(value: &FlinkClusterStatus) -> Self {
            value.clone()
        }
    }

    ///FlinkJobManager
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "cpus": 0.3,
    ///      "mem": 1024
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "cpus",
    ///    "mem"
    ///  ],
    ///  "properties": {
    ///    "cpus": {
    ///      "description": "CPU quota for the Flink job manager (minimum 0.3 =
    /// 30% of 1 CPU)",
    ///      "examples": [
    ///        0.3
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.3
    ///    },
    ///    "mem": {
    ///      "description": "Memory (MB) for this Flink job manager (minimum
    /// 1024 = 1 GB)",
    ///      "examples": [
    ///        1024
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 1024.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlinkJobManager {
        pub cpus: f64,
        ///Memory (MB) for this Flink job manager (minimum 1024 = 1 GB)
        pub mem: i64,
    }

    impl From<&FlinkJobManager> for FlinkJobManager {
        fn from(value: &FlinkJobManager) -> Self {
            value.clone()
        }
    }

    ///FlinkTaskManager
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "cpus": 0.3,
    ///      "instances": 2,
    ///      "mem": 3072
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "cpus",
    ///    "instances",
    ///    "mem"
    ///  ],
    ///  "properties": {
    ///    "cpus": {
    ///      "description": "CPU quota for each Flink task manager (minimum 0.3
    /// = 30% of 1 CPU)",
    ///      "examples": [
    ///        0.3
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.3
    ///    },
    ///    "instances": {
    ///      "description": "Number of Flink task managers (minimum 1)",
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 1.0
    ///    },
    ///    "mem": {
    ///      "description": "Memory (MB) for each Flink task manager (minimum
    /// 1024 = 1 GB)",
    ///      "examples": [
    ///        3072
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 1024.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FlinkTaskManager {
        pub cpus: f64,
        ///Number of Flink task managers (minimum 1)
        pub instances: std::num::NonZeroU64,
        ///Memory (MB) for each Flink task manager (minimum 1024 = 1 GB)
        pub mem: i64,
    }

    impl From<&FlinkTaskManager> for FlinkTaskManager {
        fn from(value: &FlinkTaskManager) -> Self {
            value.clone()
        }
    }

    ///GetManageByManagerTenantByTenantLimitByKindKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificatecount",
    ///    "secretcount",
    ///    "topiccount",
    ///    "partitioncount",
    ///    "consumerrate",
    ///    "producerrate",
    ///    "requestrate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum GetManageByManagerTenantByTenantLimitByKindKind {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificatecount")]
        Certificatecount,
        #[serde(rename = "secretcount")]
        Secretcount,
        #[serde(rename = "topiccount")]
        Topiccount,
        #[serde(rename = "partitioncount")]
        Partitioncount,
        #[serde(rename = "consumerrate")]
        Consumerrate,
        #[serde(rename = "producerrate")]
        Producerrate,
        #[serde(rename = "requestrate")]
        Requestrate,
    }

    impl From<&GetManageByManagerTenantByTenantLimitByKindKind>
        for GetManageByManagerTenantByTenantLimitByKindKind
    {
        fn from(value: &GetManageByManagerTenantByTenantLimitByKindKind) -> Self {
            value.clone()
        }
    }

    impl ToString for GetManageByManagerTenantByTenantLimitByKindKind {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::Certificatecount => "certificatecount".to_string(),
                Self::Secretcount => "secretcount".to_string(),
                Self::Topiccount => "topiccount".to_string(),
                Self::Partitioncount => "partitioncount".to_string(),
                Self::Consumerrate => "consumerrate".to_string(),
                Self::Producerrate => "producerrate".to_string(),
                Self::Requestrate => "requestrate".to_string(),
            }
        }
    }

    impl std::str::FromStr for GetManageByManagerTenantByTenantLimitByKindKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificatecount" => Ok(Self::Certificatecount),
                "secretcount" => Ok(Self::Secretcount),
                "topiccount" => Ok(Self::Topiccount),
                "partitioncount" => Ok(Self::Partitioncount),
                "consumerrate" => Ok(Self::Consumerrate),
                "producerrate" => Ok(Self::Producerrate),
                "requestrate" => Ok(Self::Requestrate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for GetManageByManagerTenantByTenantLimitByKindKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for GetManageByManagerTenantByTenantLimitByKindKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for GetManageByManagerTenantByTenantLimitByKindKind {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///HealthCheck
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "path": "/",
    ///      "port": 0,
    ///      "protocol": "http"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "properties": {
    ///    "path": {
    ///      "description": "The HTTP path for the health check\n",
    ///      "default": "/",
    ///      "type": "string"
    ///    },
    ///    "port": {
    ///      "description": "The TCP port for the health check\n",
    ///      "default": 7070,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    },
    ///    "protocol": {
    ///      "description": "The protocol for for the health check (http or
    /// https)\n",
    ///      "type": "string",
    ///      "enum": [
    ///        "http",
    ///        "https"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct HealthCheck {
        ///The HTTP path for the health check
        #[serde(default = "defaults::health_check_path")]
        pub path: String,
        ///The TCP port for the health check
        #[serde(default = "defaults::default_u64::<u64, 7070>")]
        pub port: u64,
        ///The protocol for for the health check (http or https)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<HealthCheckProtocol>,
    }

    impl From<&HealthCheck> for HealthCheck {
        fn from(value: &HealthCheck) -> Self {
            value.clone()
        }
    }

    ///The protocol for for the health check (http or https)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The protocol for for the health check (http or
    /// https)\n",
    ///  "type": "string",
    ///  "enum": [
    ///    "http",
    ///    "https"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum HealthCheckProtocol {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
    }

    impl From<&HealthCheckProtocol> for HealthCheckProtocol {
        fn from(value: &HealthCheckProtocol) -> Self {
            value.clone()
        }
    }

    impl ToString for HealthCheckProtocol {
        fn to_string(&self) -> String {
            match *self {
                Self::Http => "http".to_string(),
                Self::Https => "https".to_string(),
            }
        }
    }

    impl std::str::FromStr for HealthCheckProtocol {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "http" => Ok(Self::Http),
                "https" => Ok(Self::Https),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for HealthCheckProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for HealthCheckProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for HealthCheckProtocol {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///KafkaProxy
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "certificate": "certificate",
    ///      "cpus": 0.3,
    ///      "instances": 2,
    ///      "mem": 3072,
    ///      "name": "name",
    ///      "schemaStore": true,
    ///      "schemaStoreCpus": 0.1,
    ///      "schemaStoreMem": 256,
    ///      "secretNameCaChain": "secretNameCaChain",
    ///      "validations": [
    ///        {
    ///          "commonName": "commonName",
    ///          "country": "country",
    ///          "locality": "locality",
    ///          "organization": "organization",
    ///          "organizationalUnit": "organizationalUnit",
    ///          "province": "province",
    ///          "subjectType": "subjectType"
    ///        },
    ///        {
    ///          "commonName": "commonName",
    ///          "country": "country",
    ///          "locality": "locality",
    ///          "organization": "organization",
    ///          "organizationalUnit": "organizationalUnit",
    ///          "province": "province",
    ///          "subjectType": "subjectType"
    ///        }
    ///      ],
    ///      "zone": "internal"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "certificate",
    ///    "cpus",
    ///    "instances",
    ///    "mem",
    ///    "secretNameCaChain",
    ///    "zone"
    ///  ],
    ///  "properties": {
    ///    "certificate": {
    ///      "description": "Secret name with the server certificate",
    ///      "type": "string"
    ///    },
    ///    "cpus": {
    ///      "description": "CPU quota for each Kafka Proxy (minimum 0.3 = 30%
    /// of 1 CPU)",
    ///      "examples": [
    ///        0.3
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.3
    ///    },
    ///    "instances": {
    ///      "description": "Number of instances",
    ///      "examples": [
    ///        2
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 1.0
    ///    },
    ///    "mem": {
    ///      "description": "Memory (MB) for each Kafka Proxy (minimum 1024 = 1
    /// GB)",
    ///      "examples": [
    ///        3072
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 1024.0
    ///    },
    ///    "name": {
    ///      "description": "Name of the new Kafka Proxy",
    ///      "type": "string"
    ///    },
    ///    "schemaStore": {
    ///      "description": "Set to True no enable Schema Store",
    ///      "type": "boolean"
    ///    },
    ///    "schemaStoreCpus": {
    ///      "description": "CPU quota for Schema Store (minimum 0.3 = 30% of 1
    /// CPU)",
    ///      "examples": [
    ///        0.1
    ///      ],
    ///      "type": "number",
    ///      "minimum": 0.1
    ///    },
    ///    "schemaStoreMem": {
    ///      "description": "Memory (MB) for Schema Store (minimum 256MB)",
    ///      "examples": [
    ///        256
    ///      ],
    ///      "type": "integer",
    ///      "minimum": 256.0
    ///    },
    ///    "secretNameCaChain": {
    ///      "description": "Secret name containing the Ca Cert",
    ///      "type": "string"
    ///    },
    ///    "validations": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Validations"
    ///      }
    ///    },
    ///    "zone": {
    ///      "description": "Available networks on this platform",
    ///      "type": "string",
    ///      "enum": [
    ///        "internal",
    ///        "public"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct KafkaProxy {
        ///Secret name with the server certificate
        pub certificate: String,
        pub cpus: f64,
        ///Number of instances
        pub instances: std::num::NonZeroU64,
        ///Memory (MB) for each Kafka Proxy (minimum 1024 = 1 GB)
        pub mem: i64,
        ///Name of the new Kafka Proxy
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Set to True no enable Schema Store
        #[serde(
            rename = "schemaStore",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub schema_store: Option<bool>,
        #[serde(
            rename = "schemaStoreCpus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub schema_store_cpus: Option<f64>,
        ///Memory (MB) for Schema Store (minimum 256MB)
        #[serde(
            rename = "schemaStoreMem",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub schema_store_mem: Option<i64>,
        ///Secret name containing the Ca Cert
        #[serde(rename = "secretNameCaChain")]
        pub secret_name_ca_chain: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub validations: Vec<Validations>,
        ///Available networks on this platform
        pub zone: KafkaProxyZone,
    }

    impl From<&KafkaProxy> for KafkaProxy {
        fn from(value: &KafkaProxy) -> Self {
            value.clone()
        }
    }

    ///KafkaProxyStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/KafkaProxy"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct KafkaProxyStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<KafkaProxy>,
        pub status: AllocationStatus,
    }

    impl From<&KafkaProxyStatus> for KafkaProxyStatus {
        fn from(value: &KafkaProxyStatus) -> Self {
            value.clone()
        }
    }

    ///client certificate validations, only non empty values taken in account,
    /// no values means no validation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "client certificate validations, only non empty values
    /// taken in account, no values means no validation",
    ///  "type": "object",
    ///  "required": [
    ///    "subjectType"
    ///  ],
    ///  "properties": {
    ///    "commonName": {
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "type": "string"
    ///    },
    ///    "locality": {
    ///      "type": "string"
    ///    },
    ///    "organization": {
    ///      "type": "string"
    ///    },
    ///    "organizationalUnit": {
    ///      "type": "string"
    ///    },
    ///    "province": {
    ///      "type": "string"
    ///    },
    ///    "subjectType": {
    ///      "description": "EXACT for exact match, PATTERN for pattern match",
    ///      "type": "string",
    ///      "enum": [
    ///        "EXACT",
    ///        "PATTERN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct KafkaProxyValidation {
        #[serde(
            rename = "commonName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub common_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub organization: Option<String>,
        #[serde(
            rename = "organizationalUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub organizational_unit: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub province: Option<String>,
        ///EXACT for exact match, PATTERN for pattern match
        #[serde(rename = "subjectType")]
        pub subject_type: KafkaProxyValidationSubjectType,
    }

    impl From<&KafkaProxyValidation> for KafkaProxyValidation {
        fn from(value: &KafkaProxyValidation) -> Self {
            value.clone()
        }
    }

    ///EXACT for exact match, PATTERN for pattern match
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "EXACT for exact match, PATTERN for pattern match",
    ///  "type": "string",
    ///  "enum": [
    ///    "EXACT",
    ///    "PATTERN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum KafkaProxyValidationSubjectType {
        #[serde(rename = "EXACT")]
        Exact,
        #[serde(rename = "PATTERN")]
        Pattern,
    }

    impl From<&KafkaProxyValidationSubjectType> for KafkaProxyValidationSubjectType {
        fn from(value: &KafkaProxyValidationSubjectType) -> Self {
            value.clone()
        }
    }

    impl ToString for KafkaProxyValidationSubjectType {
        fn to_string(&self) -> String {
            match *self {
                Self::Exact => "EXACT".to_string(),
                Self::Pattern => "PATTERN".to_string(),
            }
        }
    }

    impl std::str::FromStr for KafkaProxyValidationSubjectType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "EXACT" => Ok(Self::Exact),
                "PATTERN" => Ok(Self::Pattern),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for KafkaProxyValidationSubjectType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for KafkaProxyValidationSubjectType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for KafkaProxyValidationSubjectType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Available networks on this platform
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Available networks on this platform",
    ///  "type": "string",
    ///  "enum": [
    ///    "internal",
    ///    "public"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum KafkaProxyZone {
        #[serde(rename = "internal")]
        Internal,
        #[serde(rename = "public")]
        Public,
    }

    impl From<&KafkaProxyZone> for KafkaProxyZone {
        fn from(value: &KafkaProxyZone) -> Self {
            value.clone()
        }
    }

    impl ToString for KafkaProxyZone {
        fn to_string(&self) -> String {
            match *self {
                Self::Internal => "internal".to_string(),
                Self::Public => "public".to_string(),
            }
        }
    }

    impl std::str::FromStr for KafkaProxyZone {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "internal" => Ok(Self::Internal),
                "public" => Ok(Self::Public),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for KafkaProxyZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for KafkaProxyZone {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for KafkaProxyZone {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueCpu"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueMem"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueCertificateCount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueSecretCount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueTopicCount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValuePartitionCount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueConsumerRate"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueProducerRate"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueRequestRate"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(untagged)]
    pub enum LimitValue {
        Cpu(LimitValueCpu),
        Mem(LimitValueMem),
        CertificateCount(LimitValueCertificateCount),
        SecretCount(LimitValueSecretCount),
        TopicCount(LimitValueTopicCount),
        PartitionCount(LimitValuePartitionCount),
        ConsumerRate(LimitValueConsumerRate),
        ProducerRate(LimitValueProducerRate),
        RequestRate(LimitValueRequestRate),
    }

    impl From<&LimitValue> for LimitValue {
        fn from(value: &LimitValue) -> Self {
            value.clone()
        }
    }

    impl From<LimitValueCpu> for LimitValue {
        fn from(value: LimitValueCpu) -> Self {
            Self::Cpu(value)
        }
    }

    impl From<LimitValueMem> for LimitValue {
        fn from(value: LimitValueMem) -> Self {
            Self::Mem(value)
        }
    }

    impl From<LimitValueCertificateCount> for LimitValue {
        fn from(value: LimitValueCertificateCount) -> Self {
            Self::CertificateCount(value)
        }
    }

    impl From<LimitValueSecretCount> for LimitValue {
        fn from(value: LimitValueSecretCount) -> Self {
            Self::SecretCount(value)
        }
    }

    impl From<LimitValueTopicCount> for LimitValue {
        fn from(value: LimitValueTopicCount) -> Self {
            Self::TopicCount(value)
        }
    }

    impl From<LimitValuePartitionCount> for LimitValue {
        fn from(value: LimitValuePartitionCount) -> Self {
            Self::PartitionCount(value)
        }
    }

    impl From<LimitValueConsumerRate> for LimitValue {
        fn from(value: LimitValueConsumerRate) -> Self {
            Self::ConsumerRate(value)
        }
    }

    impl From<LimitValueProducerRate> for LimitValue {
        fn from(value: LimitValueProducerRate) -> Self {
            Self::ProducerRate(value)
        }
    }

    impl From<LimitValueRequestRate> for LimitValue {
        fn from(value: LimitValueRequestRate) -> Self {
            Self::RequestRate(value)
        }
    }

    ///LimitValueCertificateCount
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueCertificateCount_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueCertificateCount {
        pub name: LimitValueCertificateCountName,
        ///The number of certificates available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValueCertificateCount> for LimitValueCertificateCount {
        fn from(value: &LimitValueCertificateCount) -> Self {
            value.clone()
        }
    }

    ///LimitValueCertificateCountAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The number of certificates available for the
    /// managed tenant",
    ///      "type": "integer",
    ///      "maximum": 40.0,
    ///      "minimum": 1.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueCertificateCountAllOf {
        ///The number of certificates available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValueCertificateCountAllOf> for LimitValueCertificateCountAllOf {
        fn from(value: &LimitValueCertificateCountAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueCertificateCountName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueCertificateCountName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueCertificateCountName> for LimitValueCertificateCountName {
        fn from(value: &LimitValueCertificateCountName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueCertificateCountName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueCertificateCountName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueCertificateCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueCertificateCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueCertificateCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueConsumerRate
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueConsumerRate_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueConsumerRate {
        pub name: LimitValueConsumerRateName,
        ///The maximum allowed consumer rate (bytes/sec)
        pub value: i64,
    }

    impl From<&LimitValueConsumerRate> for LimitValueConsumerRate {
        fn from(value: &LimitValueConsumerRate) -> Self {
            value.clone()
        }
    }

    ///LimitValueConsumerRateAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The maximum allowed consumer rate (bytes/sec)",
    ///      "type": "integer",
    ///      "maximum": 1250000000.0,
    ///      "minimum": 1048576.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueConsumerRateAllOf {
        ///The maximum allowed consumer rate (bytes/sec)
        pub value: i64,
    }

    impl From<&LimitValueConsumerRateAllOf> for LimitValueConsumerRateAllOf {
        fn from(value: &LimitValueConsumerRateAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueConsumerRateName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueConsumerRateName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueConsumerRateName> for LimitValueConsumerRateName {
        fn from(value: &LimitValueConsumerRateName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueConsumerRateName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueConsumerRateName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueConsumerRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueConsumerRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueConsumerRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueCpu
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueCpu_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueCpu {
        pub name: LimitValueCpuName,
        pub value: f64,
    }

    impl From<&LimitValueCpu> for LimitValueCpu {
        fn from(value: &LimitValueCpu) -> Self {
            value.clone()
        }
    }

    ///LimitValueCpuAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The number of CPUs to provision for the managed
    /// tenant (factions of a vCPU core, 1.0 equals 1 vCPU)",
    ///      "type": "number",
    ///      "multipleOf": 0.01,
    ///      "maximum": 16.0,
    ///      "minimum": 0.01
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueCpuAllOf {
        pub value: f64,
    }

    impl From<&LimitValueCpuAllOf> for LimitValueCpuAllOf {
        fn from(value: &LimitValueCpuAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueCpuName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueCpuName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueCpuName> for LimitValueCpuName {
        fn from(value: &LimitValueCpuName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueCpuName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueCpuName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueCpuName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueCpuName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueCpuName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueMem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueMem_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueMem {
        pub name: LimitValueMemName,
        ///The amount of memory available for the managed tenant (MiB)
        pub value: i64,
    }

    impl From<&LimitValueMem> for LimitValueMem {
        fn from(value: &LimitValueMem) -> Self {
            value.clone()
        }
    }

    ///LimitValueMemAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The amount of memory available for the managed
    /// tenant (MiB)",
    ///      "type": "integer",
    ///      "maximum": 131072.0,
    ///      "minimum": 1.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueMemAllOf {
        ///The amount of memory available for the managed tenant (MiB)
        pub value: i64,
    }

    impl From<&LimitValueMemAllOf> for LimitValueMemAllOf {
        fn from(value: &LimitValueMemAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueMemName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueMemName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueMemName> for LimitValueMemName {
        fn from(value: &LimitValueMemName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueMemName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueMemName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueMemName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueMemName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueMemName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValuePartitionCount
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValuePartitionCount_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValuePartitionCount {
        pub name: LimitValuePartitionCountName,
        ///The number of partitions available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValuePartitionCount> for LimitValuePartitionCount {
        fn from(value: &LimitValuePartitionCount) -> Self {
            value.clone()
        }
    }

    ///LimitValuePartitionCountAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The number of partitions available for the managed
    /// tenant",
    ///      "type": "integer",
    ///      "maximum": 40.0,
    ///      "minimum": 1.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValuePartitionCountAllOf {
        ///The number of partitions available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValuePartitionCountAllOf> for LimitValuePartitionCountAllOf {
        fn from(value: &LimitValuePartitionCountAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValuePartitionCountName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValuePartitionCountName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValuePartitionCountName> for LimitValuePartitionCountName {
        fn from(value: &LimitValuePartitionCountName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValuePartitionCountName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValuePartitionCountName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValuePartitionCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValuePartitionCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValuePartitionCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueProducerRate
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueProducerRate_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueProducerRate {
        pub name: LimitValueProducerRateName,
        ///The maximum allowed producer rate (bytes/sec)
        pub value: i64,
    }

    impl From<&LimitValueProducerRate> for LimitValueProducerRate {
        fn from(value: &LimitValueProducerRate) -> Self {
            value.clone()
        }
    }

    ///LimitValueProducerRateAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The maximum allowed producer rate (bytes/sec)",
    ///      "type": "integer",
    ///      "maximum": 1250000000.0,
    ///      "minimum": 1048576.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueProducerRateAllOf {
        ///The maximum allowed producer rate (bytes/sec)
        pub value: i64,
    }

    impl From<&LimitValueProducerRateAllOf> for LimitValueProducerRateAllOf {
        fn from(value: &LimitValueProducerRateAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueProducerRateName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueProducerRateName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueProducerRateName> for LimitValueProducerRateName {
        fn from(value: &LimitValueProducerRateName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueProducerRateName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueProducerRateName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueProducerRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueProducerRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueProducerRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueRequestRate
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueRequestRate_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueRequestRate {
        pub name: LimitValueRequestRateName,
        ///The maximum allowed request rate (%)
        pub value: i64,
    }

    impl From<&LimitValueRequestRate> for LimitValueRequestRate {
        fn from(value: &LimitValueRequestRate) -> Self {
            value.clone()
        }
    }

    ///LimitValueRequestRateAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The maximum allowed request rate (%)",
    ///      "type": "integer",
    ///      "maximum": 100.0,
    ///      "minimum": 1.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueRequestRateAllOf {
        ///The maximum allowed request rate (%)
        pub value: i64,
    }

    impl From<&LimitValueRequestRateAllOf> for LimitValueRequestRateAllOf {
        fn from(value: &LimitValueRequestRateAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueRequestRateName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueRequestRateName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueRequestRateName> for LimitValueRequestRateName {
        fn from(value: &LimitValueRequestRateName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueRequestRateName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueRequestRateName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueRequestRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueRequestRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueRequestRateName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueSecretCount
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueSecretCount_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueSecretCount {
        pub name: LimitValueSecretCountName,
        ///The number of secrets available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValueSecretCount> for LimitValueSecretCount {
        fn from(value: &LimitValueSecretCount) -> Self {
            value.clone()
        }
    }

    ///LimitValueSecretCountAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The number of secrets available for the managed
    /// tenant",
    ///      "type": "integer",
    ///      "maximum": 40.0,
    ///      "minimum": 1.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueSecretCountAllOf {
        ///The number of secrets available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValueSecretCountAllOf> for LimitValueSecretCountAllOf {
        fn from(value: &LimitValueSecretCountAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueSecretCountName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueSecretCountName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueSecretCountName> for LimitValueSecretCountName {
        fn from(value: &LimitValueSecretCountName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueSecretCountName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueSecretCountName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueSecretCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueSecretCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueSecretCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///LimitValueTopicCount
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/BaseLimitValue"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/LimitValueTopicCount_allOf"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueTopicCount {
        pub name: LimitValueTopicCountName,
        ///The number of topics available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValueTopicCount> for LimitValueTopicCount {
        fn from(value: &LimitValueTopicCount) -> Self {
            value.clone()
        }
    }

    ///LimitValueTopicCountAllOf
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "description": "The number of topics available for the managed
    /// tenant",
    ///      "type": "integer",
    ///      "maximum": 40.0,
    ///      "minimum": 1.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct LimitValueTopicCountAllOf {
        ///The number of topics available for the managed tenant
        pub value: i64,
    }

    impl From<&LimitValueTopicCountAllOf> for LimitValueTopicCountAllOf {
        fn from(value: &LimitValueTopicCountAllOf) -> Self {
            value.clone()
        }
    }

    ///LimitValueTopicCountName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificateCount",
    ///    "secretCount",
    ///    "topicCount",
    ///    "partitionCount",
    ///    "consumerRate",
    ///    "producerRate",
    ///    "requestRate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum LimitValueTopicCountName {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificateCount")]
        CertificateCount,
        #[serde(rename = "secretCount")]
        SecretCount,
        #[serde(rename = "topicCount")]
        TopicCount,
        #[serde(rename = "partitionCount")]
        PartitionCount,
        #[serde(rename = "consumerRate")]
        ConsumerRate,
        #[serde(rename = "producerRate")]
        ProducerRate,
        #[serde(rename = "requestRate")]
        RequestRate,
    }

    impl From<&LimitValueTopicCountName> for LimitValueTopicCountName {
        fn from(value: &LimitValueTopicCountName) -> Self {
            value.clone()
        }
    }

    impl ToString for LimitValueTopicCountName {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::CertificateCount => "certificateCount".to_string(),
                Self::SecretCount => "secretCount".to_string(),
                Self::TopicCount => "topicCount".to_string(),
                Self::PartitionCount => "partitionCount".to_string(),
                Self::ConsumerRate => "consumerRate".to_string(),
                Self::ProducerRate => "producerRate".to_string(),
                Self::RequestRate => "requestRate".to_string(),
            }
        }
    }

    impl std::str::FromStr for LimitValueTopicCountName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificateCount" => Ok(Self::CertificateCount),
                "secretCount" => Ok(Self::SecretCount),
                "topicCount" => Ok(Self::TopicCount),
                "partitionCount" => Ok(Self::PartitionCount),
                "consumerRate" => Ok(Self::ConsumerRate),
                "producerRate" => Ok(Self::ProducerRate),
                "requestRate" => Ok(Self::RequestRate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for LimitValueTopicCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for LimitValueTopicCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for LimitValueTopicCountName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ManagedTenant
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "manager": "manager",
    ///      "name": "name",
    ///      "services": [
    ///        {
    ///          "enabled": true,
    ///          "name": "vpn"
    ///        },
    ///        {
    ///          "enabled": true,
    ///          "name": "vpn"
    ///        }
    ///      ]
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "manager",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "manager": {
    ///      "description": "Name of the tenant that is acting as manager for
    /// this tenant.  \nMust be identical to the `manager` parameter in the
    /// path.\n",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "Name of the tenant.  Must be identical to the
    /// tenant name used in the path.",
    ///      "type": "string"
    ///    },
    ///    "services": {
    ///      "description": "List of services that are enabled for this tenant.
    /// At this point, `monitoring` is a requirement (it's \n`enabled` value
    /// must be `true`).  The default values for `tracing` and `vpn` are both
    /// `false`.  The `vpn`\nservice is only available on some platforms.
    /// Requesting it on a platform that doesn't support it will \ncause the
    /// request to be rejected.\n",
    ///      "default": [
    ///        {
    ///          "enabled": true,
    ///          "name": "monitoring"
    ///        },
    ///        {
    ///          "enabled": false,
    ///          "name": "vpn"
    ///        },
    ///        {
    ///          "enabled": false,
    ///          "name": "tracing"
    ///        }
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ManagedTenant_services"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ManagedTenant {
        ///Name of the tenant that is acting as manager for this tenant.  
        ///Must be identical to the `manager` parameter in the path.
        pub manager: String,
        ///Name of the tenant.  Must be identical to the tenant name used in
        /// the path.
        pub name: String,
        ///List of services that are enabled for this tenant.  At this point,
        /// `monitoring` is a requirement (it's `enabled` value must be
        /// `true`).  The default values for `tracing` and `vpn` are both
        /// `false`.  The `vpn` service is only available on some
        /// platforms.  Requesting it on a platform that doesn't support it will
        /// cause the request to be rejected.
        #[serde(default = "defaults::managed_tenant_services")]
        pub services: Vec<ManagedTenantServices>,
    }

    impl From<&ManagedTenant> for ManagedTenant {
        fn from(value: &ManagedTenant) -> Self {
            value.clone()
        }
    }

    ///ManagedTenantServices
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "enabled": true,
    ///      "name": "vpn"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "enabled",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string",
    ///      "enum": [
    ///        "vpn",
    ///        "tracing",
    ///        "monitoring"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ManagedTenantServices {
        pub enabled: bool,
        pub name: ManagedTenantServicesName,
    }

    impl From<&ManagedTenantServices> for ManagedTenantServices {
        fn from(value: &ManagedTenantServices) -> Self {
            value.clone()
        }
    }

    ///ManagedTenantServicesName
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "vpn",
    ///    "tracing",
    ///    "monitoring"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum ManagedTenantServicesName {
        #[serde(rename = "vpn")]
        Vpn,
        #[serde(rename = "tracing")]
        Tracing,
        #[serde(rename = "monitoring")]
        Monitoring,
    }

    impl From<&ManagedTenantServicesName> for ManagedTenantServicesName {
        fn from(value: &ManagedTenantServicesName) -> Self {
            value.clone()
        }
    }

    impl ToString for ManagedTenantServicesName {
        fn to_string(&self) -> String {
            match *self {
                Self::Vpn => "vpn".to_string(),
                Self::Tracing => "tracing".to_string(),
                Self::Monitoring => "monitoring".to_string(),
            }
        }
    }

    impl std::str::FromStr for ManagedTenantServicesName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "vpn" => Ok(Self::Vpn),
                "tracing" => Ok(Self::Tracing),
                "monitoring" => Ok(Self::Monitoring),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ManagedTenantServicesName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ManagedTenantServicesName {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ManagedTenantServicesName {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///metrics endpoint which will be scraped by the platform.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "metrics endpoint which will be scraped by the
    /// platform.",
    ///  "examples": [
    ///    {
    ///      "path": "/metrics",
    ///      "port": 0
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "properties": {
    ///    "path": {
    ///      "description": "The HTTP path for the metrics endpoint\n",
    ///      "default": "/metrics",
    ///      "type": "string"
    ///    },
    ///    "port": {
    ///      "description": "The TCP port for the metrics endpoint\n",
    ///      "default": 7070,
    ///      "type": "integer",
    ///      "minimum": 0.0
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Metrics {
        ///The HTTP path for the metrics endpoint
        #[serde(default = "defaults::metrics_path")]
        pub path: String,
        ///The TCP port for the metrics endpoint
        #[serde(default = "defaults::default_u64::<u64, 7070>")]
        pub port: u64,
    }

    impl From<&Metrics> for Metrics {
        fn from(value: &Metrics) -> Self {
            value.clone()
        }
    }

    ///Notification
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "args": {
    ///        "key": "args"
    ///      },
    ///      "message": "message",
    ///      "remove": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "remove"
    ///  ],
    ///  "properties": {
    ///    "args": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "remove": {
    ///      "description": "true if the notification has to do with removal of the allocation, false if it relates to creation/update of the resource",
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Notification {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub args: std::collections::HashMap<String, String>,
        pub message: String,
        ///true if the notification has to do with removal of the allocation,
        /// false if it relates to creation/update of the resource
        pub remove: bool,
    }

    impl From<&Notification> for Notification {
        fn from(value: &Notification) -> Self {
            value.clone()
        }
    }

    ///PathSpec
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "prefix": "prefix"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "prefix"
    ///  ],
    ///  "properties": {
    ///    "prefix": {
    ///      "description": "The path prefix (starting with `/`, ending without
    /// `/`) that will be matched for routing to this service.",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PathSpec {
        ///The path prefix (starting with `/`, ending without `/`) that will be
        /// matched for routing to this service.
        pub prefix: String,
    }

    impl From<&PathSpec> for PathSpec {
        fn from(value: &PathSpec) -> Self {
            value.clone()
        }
    }

    ///PortMapping
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "auth": "auth",
    ///      "mode": "mode",
    ///      "paths": [
    ///        {
    ///          "prefix": "prefix"
    ///        },
    ///        {
    ///          "prefix": "prefix"
    ///        }
    ///      ],
    ///      "serviceGroup": "serviceGroup",
    ///      "tls": "auto",
    ///      "vhost": "vhost",
    ///      "whitelist": "whitelist"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "properties": {
    ///    "auth": {
    ///      "description": "TODO",
    ///      "type": "string"
    ///    },
    ///    "mode": {
    ///      "description": "Routing mode. The allowed values are:\n  * `http`
    /// (default if this property is omitted). HTTP routing and TLS termination
    /// are done by the platform. In this case, the `tls` and (optionally)
    /// `paths` settings should be configured as well.\n  * `tcp/<endpoint>`.
    /// The platform only does plain TCP routing, with TLS pass-through. When
    /// set, the `tls` and `paths` settings are ignored. The application is
    /// responsible for TLS termination and certificate management. There are
    /// various possible values for `<endpoint>` that may appear when listing
    /// allocation configurations, but the only value that is allowed to be set
    /// in regular application allocations is `tcp/https`.\n    * `tcp/https`.
    /// Any traffic arriving on `<vhost>:443` will be forwarded (TLS included)
    /// to the service.\n    * `tcp/kafka-proxy` is used by Kafka Proxies. This
    /// endpoint is auto-configured by the platform when allocating a Kafka
    /// Proxy application and should *not* be used when allocating regular
    /// applications.\n    * `tcp/vpn-tcp` is used by a VPN application. This
    /// endpoint is auto-configured by the platform when allocating a VPN
    /// application and should *not* be used when allocating regular
    /// applications.\n",
    ///      "type": "string"
    ///    },
    ///    "paths": {
    ///      "description": "The paths which are allowed on the associated
    /// vhost",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PathSpec"
    ///      }
    ///    },
    ///    "serviceGroup": {
    ///      "description": "To load balance traffic between different services,
    /// use this optional field to put those services in the same service group.
    /// Choose any name consisting of all lowercase letters.",
    ///      "type": "string"
    ///    },
    ///    "tls": {
    ///      "description": "The default is 'auto', indicating that the port
    /// will only accept secured connections. Put this to 'none' if you do not
    /// want the service to have a secure endpoint.",
    ///      "type": "string",
    ///      "enum": [
    ///        "auto",
    ///        "none"
    ///      ]
    ///    },
    ///    "vhost": {
    ///      "description": "The host name that needs to be assigned to this
    /// port (for multiple names, separate them with commas)",
    ///      "type": "string"
    ///    },
    ///    "whitelist": {
    ///      "description": "Put ip addresses or ip ranges that can call this
    /// service here (for multiple addresses, separate them with spaces)",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PortMapping {
        ///TODO
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth: Option<String>,
        ///Routing mode. The allowed values are:
        ///  * `http` (default if this property is omitted). HTTP routing and
        ///    TLS termination are done by the platform. In this case, the `tls`
        ///    and (optionally) `paths` settings should be configured as well.
        ///  * `tcp/<endpoint>`. The platform only does plain TCP routing, with
        ///    TLS pass-through. When set, the `tls` and `paths` settings are
        ///    ignored. The application is responsible for TLS termination and
        ///    certificate management. There are various possible values for
        ///    `<endpoint>` that may appear when listing allocation
        ///    configurations, but the only value that is allowed to be set in
        ///    regular application allocations is `tcp/https`.
        ///    * `tcp/https`. Any traffic arriving on `<vhost>:443` will be
        ///      forwarded (TLS included) to the service.
        ///    * `tcp/kafka-proxy` is used by Kafka Proxies. This endpoint is
        ///      auto-configured by the platform when allocating a Kafka Proxy
        ///      application and should *not* be used when allocating regular
        ///      applications.
        ///    * `tcp/vpn-tcp` is used by a VPN application. This endpoint is
        ///      auto-configured by the platform when allocating a VPN
        ///      application and should *not* be used when allocating regular
        ///      applications.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mode: Option<String>,
        ///The paths which are allowed on the associated vhost
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub paths: Vec<PathSpec>,
        ///To load balance traffic between different services, use this
        /// optional field to put those services in the same service group.
        /// Choose any name consisting of all lowercase letters.
        #[serde(
            rename = "serviceGroup",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_group: Option<String>,
        ///The default is 'auto', indicating that the port will only accept
        /// secured connections. Put this to 'none' if you do not want the
        /// service to have a secure endpoint.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tls: Option<PortMappingTls>,
        ///The host name that needs to be assigned to this port (for multiple
        /// names, separate them with commas)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vhost: Option<String>,
        ///Put ip addresses or ip ranges that can call this service here (for
        /// multiple addresses, separate them with spaces)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub whitelist: Option<String>,
    }

    impl From<&PortMapping> for PortMapping {
        fn from(value: &PortMapping) -> Self {
            value.clone()
        }
    }

    ///The default is 'auto', indicating that the port will only accept secured
    /// connections. Put this to 'none' if you do not want the service to have a
    /// secure endpoint.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The default is 'auto', indicating that the port will
    /// only accept secured connections. Put this to 'none' if you do not want
    /// the service to have a secure endpoint.",
    ///  "type": "string",
    ///  "enum": [
    ///    "auto",
    ///    "none"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum PortMappingTls {
        #[serde(rename = "auto")]
        Auto,
        #[serde(rename = "none")]
        None,
    }

    impl From<&PortMappingTls> for PortMappingTls {
        fn from(value: &PortMappingTls) -> Self {
            value.clone()
        }
    }

    impl ToString for PortMappingTls {
        fn to_string(&self) -> String {
            match *self {
                Self::Auto => "auto".to_string(),
                Self::None => "none".to_string(),
            }
        }
    }

    impl std::str::FromStr for PortMappingTls {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "auto" => Ok(Self::Auto),
                "none" => Ok(Self::None),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PortMappingTls {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PortMappingTls {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PortMappingTls {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///PutManageByManagerTenantByTenantLimitByKindKind
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "cpu",
    ///    "mem",
    ///    "certificatecount",
    ///    "secretcount",
    ///    "topiccount",
    ///    "partitioncount",
    ///    "consumerrate",
    ///    "producerrate",
    ///    "requestrate"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum PutManageByManagerTenantByTenantLimitByKindKind {
        #[serde(rename = "cpu")]
        Cpu,
        #[serde(rename = "mem")]
        Mem,
        #[serde(rename = "certificatecount")]
        Certificatecount,
        #[serde(rename = "secretcount")]
        Secretcount,
        #[serde(rename = "topiccount")]
        Topiccount,
        #[serde(rename = "partitioncount")]
        Partitioncount,
        #[serde(rename = "consumerrate")]
        Consumerrate,
        #[serde(rename = "producerrate")]
        Producerrate,
        #[serde(rename = "requestrate")]
        Requestrate,
    }

    impl From<&PutManageByManagerTenantByTenantLimitByKindKind>
        for PutManageByManagerTenantByTenantLimitByKindKind
    {
        fn from(value: &PutManageByManagerTenantByTenantLimitByKindKind) -> Self {
            value.clone()
        }
    }

    impl ToString for PutManageByManagerTenantByTenantLimitByKindKind {
        fn to_string(&self) -> String {
            match *self {
                Self::Cpu => "cpu".to_string(),
                Self::Mem => "mem".to_string(),
                Self::Certificatecount => "certificatecount".to_string(),
                Self::Secretcount => "secretcount".to_string(),
                Self::Topiccount => "topiccount".to_string(),
                Self::Partitioncount => "partitioncount".to_string(),
                Self::Consumerrate => "consumerrate".to_string(),
                Self::Producerrate => "producerrate".to_string(),
                Self::Requestrate => "requestrate".to_string(),
            }
        }
    }

    impl std::str::FromStr for PutManageByManagerTenantByTenantLimitByKindKind {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cpu" => Ok(Self::Cpu),
                "mem" => Ok(Self::Mem),
                "certificatecount" => Ok(Self::Certificatecount),
                "secretcount" => Ok(Self::Secretcount),
                "topiccount" => Ok(Self::Topiccount),
                "partitioncount" => Ok(Self::Partitioncount),
                "consumerrate" => Ok(Self::Consumerrate),
                "producerrate" => Ok(Self::Producerrate),
                "requestrate" => Ok(Self::Requestrate),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PutManageByManagerTenantByTenantLimitByKindKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PutManageByManagerTenantByTenantLimitByKindKind {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PutManageByManagerTenantByTenantLimitByKindKind {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Secret
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "name": "name",
    ///      "value": "value"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Secret {
        pub name: String,
        pub value: String,
    }

    impl From<&Secret> for Secret {
        fn from(value: &Secret) -> Self {
            value.clone()
        }
    }

    ///Task
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "healthy": true,
    ///      "host": "10.0.2.36",
    ///      "lastUpdate": 1638980430,
    ///      "stagedAt": "2017-12-07T10:53:46.643Z",
    ///      "startedAt": "2017-12-07T10:55:41.765Z",
    ///      "state": "RUNNING",
    ///      "stoppedAt": "2017-12-07T10:58:41.765Z"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "host",
    ///    "stagedAt",
    ///    "startedAt",
    ///    "state"
    ///  ],
    ///  "properties": {
    ///    "healthy": {
    ///      "description": "false or true depending on health checks (empty if
    /// no health checks)\n",
    ///      "type": "boolean"
    ///    },
    ///    "host": {
    ///      "description": "The IP address of the host the task is running on
    /// (not the IP address of the task itself)\n",
    ///      "type": "string",
    ///      "format": "ipv4"
    ///    },
    ///    "lastUpdate": {
    ///      "description": "Timestamp of the last time the task was updated",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "logs": {
    ///      "description": "Optional link to the latest log dump for this
    /// task",
    ///      "type": "string",
    ///      "format": "url"
    ///    },
    ///    "stagedAt": {
    ///      "description": "Staging time of the task",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "startedAt": {
    ///      "description": "Start time of the task",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "state": {
    ///      "description": "The state the task is in",
    ///      "type": "string",
    ///      "enum": [
    ///        "DROPPED",
    ///        "ERROR",
    ///        "FAILED",
    ///        "FINISHED",
    ///        "GONE",
    ///        "GONE_BY_OPERATOR",
    ///        "KILLED",
    ///        "KILLING",
    ///        "LOST",
    ///        "RUNNING",
    ///        "STAGING",
    ///        "STARTING",
    ///        "UNKNOWN",
    ///        "UNREACHABLE"
    ///      ]
    ///    },
    ///    "stoppedAt": {
    ///      "description": "Stopped time of the task",
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Task {
        ///false or true depending on health checks (empty if no health checks)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub healthy: Option<bool>,
        ///The IP address of the host the task is running on (not the IP
        /// address of the task itself)
        pub host: std::net::Ipv4Addr,
        ///Timestamp of the last time the task was updated
        #[serde(
            rename = "lastUpdate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_update: Option<i64>,
        ///Optional link to the latest log dump for this task
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logs: Option<String>,
        ///Staging time of the task
        #[serde(rename = "stagedAt")]
        pub staged_at: chrono::DateTime<chrono::offset::Utc>,
        ///Start time of the task
        #[serde(rename = "startedAt")]
        pub started_at: chrono::DateTime<chrono::offset::Utc>,
        ///The state the task is in
        pub state: TaskState,
        ///Stopped time of the task
        #[serde(rename = "stoppedAt", default, skip_serializing_if = "Option::is_none")]
        pub stopped_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&Task> for Task {
        fn from(value: &Task) -> Self {
            value.clone()
        }
    }

    ///The state the task is in
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The state the task is in",
    ///  "type": "string",
    ///  "enum": [
    ///    "DROPPED",
    ///    "ERROR",
    ///    "FAILED",
    ///    "FINISHED",
    ///    "GONE",
    ///    "GONE_BY_OPERATOR",
    ///    "KILLED",
    ///    "KILLING",
    ///    "LOST",
    ///    "RUNNING",
    ///    "STAGING",
    ///    "STARTING",
    ///    "UNKNOWN",
    ///    "UNREACHABLE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum TaskState {
        #[serde(rename = "DROPPED")]
        Dropped,
        #[serde(rename = "ERROR")]
        Error,
        #[serde(rename = "FAILED")]
        Failed,
        #[serde(rename = "FINISHED")]
        Finished,
        #[serde(rename = "GONE")]
        Gone,
        #[serde(rename = "GONE_BY_OPERATOR")]
        GoneByOperator,
        #[serde(rename = "KILLED")]
        Killed,
        #[serde(rename = "KILLING")]
        Killing,
        #[serde(rename = "LOST")]
        Lost,
        #[serde(rename = "RUNNING")]
        Running,
        #[serde(rename = "STAGING")]
        Staging,
        #[serde(rename = "STARTING")]
        Starting,
        #[serde(rename = "UNKNOWN")]
        Unknown,
        #[serde(rename = "UNREACHABLE")]
        Unreachable,
    }

    impl From<&TaskState> for TaskState {
        fn from(value: &TaskState) -> Self {
            value.clone()
        }
    }

    impl ToString for TaskState {
        fn to_string(&self) -> String {
            match *self {
                Self::Dropped => "DROPPED".to_string(),
                Self::Error => "ERROR".to_string(),
                Self::Failed => "FAILED".to_string(),
                Self::Finished => "FINISHED".to_string(),
                Self::Gone => "GONE".to_string(),
                Self::GoneByOperator => "GONE_BY_OPERATOR".to_string(),
                Self::Killed => "KILLED".to_string(),
                Self::Killing => "KILLING".to_string(),
                Self::Lost => "LOST".to_string(),
                Self::Running => "RUNNING".to_string(),
                Self::Staging => "STAGING".to_string(),
                Self::Starting => "STARTING".to_string(),
                Self::Unknown => "UNKNOWN".to_string(),
                Self::Unreachable => "UNREACHABLE".to_string(),
            }
        }
    }

    impl std::str::FromStr for TaskState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "DROPPED" => Ok(Self::Dropped),
                "ERROR" => Ok(Self::Error),
                "FAILED" => Ok(Self::Failed),
                "FINISHED" => Ok(Self::Finished),
                "GONE" => Ok(Self::Gone),
                "GONE_BY_OPERATOR" => Ok(Self::GoneByOperator),
                "KILLED" => Ok(Self::Killed),
                "KILLING" => Ok(Self::Killing),
                "LOST" => Ok(Self::Lost),
                "RUNNING" => Ok(Self::Running),
                "STAGING" => Ok(Self::Staging),
                "STARTING" => Ok(Self::Starting),
                "UNKNOWN" => Ok(Self::Unknown),
                "UNREACHABLE" => Ok(Self::Unreachable),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for TaskState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for TaskState {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for TaskState {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///TaskStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "healthy": true,
    ///        "host": "10.0.2.36",
    ///        "lastUpdate": 1638980430,
    ///        "stagedAt": "2017-12-07T10:53:46.643Z",
    ///        "startedAt": "2017-12-07T10:55:41.765Z",
    ///        "state": "RUNNING",
    ///        "stoppedAt": "2017-12-07T10:58:41.765Z"
    ///      },
    ///      "configuration": {
    ///        "healthy": true,
    ///        "host": "10.0.2.36",
    ///        "lastUpdate": 1638980430,
    ///        "stagedAt": "2017-12-07T10:53:46.643Z",
    ///        "startedAt": "2017-12-07T10:55:41.765Z",
    ///        "state": "RUNNING",
    ///        "stoppedAt": "2017-12-07T10:58:41.765Z"
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/Task"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/Task"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TaskStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<Task>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Task>,
        pub status: AllocationStatus,
    }

    impl From<&TaskStatus> for TaskStatus {
        fn from(value: &TaskStatus) -> Self {
            value.clone()
        }
    }

    ///ThirdPartyBucketConcession
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "credentialidentifierref": "credentialidentifierref",
    ///      "credentialsecretref": "credentialsecretref",
    ///      "name": "name",
    ///      "readable": true,
    ///      "shareidentifier": "shareidentifier",
    ///      "writable": true
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "credentialidentifierref",
    ///    "credentialsecretref",
    ///    "name",
    ///    "readable",
    ///    "shareidentifier",
    ///    "writable"
    ///  ],
    ///  "properties": {
    ///    "credentialidentifierref": {
    ///      "type": "string"
    ///    },
    ///    "credentialsecretref": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "your name for this bucket owned by a third party",
    ///      "type": "string"
    ///    },
    ///    "readable": {
    ///      "type": "boolean"
    ///    },
    ///    "shareidentifier": {
    ///      "type": "string"
    ///    },
    ///    "writable": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ThirdPartyBucketConcession {
        pub credentialidentifierref: String,
        pub credentialsecretref: String,
        ///your name for this bucket owned by a third party
        pub name: String,
        pub readable: bool,
        pub shareidentifier: String,
        pub writable: bool,
    }

    impl From<&ThirdPartyBucketConcession> for ThirdPartyBucketConcession {
        fn from(value: &ThirdPartyBucketConcession) -> Self {
            value.clone()
        }
    }

    ///ThirdPartyBucketConcessionConfiguration
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "name": "name",
    ///      "shareidentifier": "shareidentifier"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "shareidentifier"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "description": "your name for this bucket owned by a third party",
    ///      "type": "string"
    ///    },
    ///    "shareidentifier": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ThirdPartyBucketConcessionConfiguration {
        ///your name for this bucket owned by a third party
        pub name: String,
        pub shareidentifier: String,
    }

    impl From<&ThirdPartyBucketConcessionConfiguration> for ThirdPartyBucketConcessionConfiguration {
        fn from(value: &ThirdPartyBucketConcessionConfiguration) -> Self {
            value.clone()
        }
    }

    ///ThirdPartyBucketConcessionRegistration
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "credentialidentifierplaintext": "credentialidentifierplaintext",
    ///      "credentialsecretplaintext": "credentialsecretplaintext",
    ///      "name": "name",
    ///      "shareidentifier": "shareidentifier"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "credentialidentifierplaintext",
    ///    "credentialsecretplaintext",
    ///    "name",
    ///    "shareidentifier"
    ///  ],
    ///  "properties": {
    ///    "credentialidentifierplaintext": {
    ///      "description": "plaintext credential identifier provided to you by
    /// the third party",
    ///      "type": "string"
    ///    },
    ///    "credentialsecretplaintext": {
    ///      "description": "plaintext secret value provided to you by the third
    /// party",
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "description": "the name you give to the third party bucket you are
    /// registering",
    ///      "type": "string"
    ///    },
    ///    "shareidentifier": {
    ///      "description": "provided to you by the third party",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ThirdPartyBucketConcessionRegistration {
        ///plaintext credential identifier provided to you by the third party
        pub credentialidentifierplaintext: String,
        ///plaintext secret value provided to you by the third party
        pub credentialsecretplaintext: String,
        ///the name you give to the third party bucket you are registering
        pub name: String,
        ///provided to you by the third party
        pub shareidentifier: String,
    }

    impl From<&ThirdPartyBucketConcessionRegistration> for ThirdPartyBucketConcessionRegistration {
        fn from(value: &ThirdPartyBucketConcessionRegistration) -> Self {
            value.clone()
        }
    }

    ///ThirdPartyBucketConcessionStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "credentialidentifierref": "credentialidentifierref",
    ///        "credentialsecretref": "credentialsecretref",
    ///        "name": "name",
    ///        "readable": true,
    ///        "shareidentifier": "shareidentifier",
    ///        "writable": true
    ///      },
    ///      "configuration": {
    ///        "name": "name",
    ///        "shareidentifier": "shareidentifier"
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/ThirdPartyBucketConcession"
    ///    },
    ///    "configuration": {
    ///      "$ref":
    /// "#/components/schemas/ThirdPartyBucketConcessionConfiguration"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ThirdPartyBucketConcessionStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<ThirdPartyBucketConcession>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<ThirdPartyBucketConcessionConfiguration>,
        pub status: AllocationStatus,
    }

    impl From<&ThirdPartyBucketConcessionStatus> for ThirdPartyBucketConcessionStatus {
        fn from(value: &ThirdPartyBucketConcessionStatus) -> Self {
            value.clone()
        }
    }

    ///Topic
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "kafkaProperties": {
    ///        "key": "kafkaProperties"
    ///      },
    ///      "partitions": 0,
    ///      "replicationFactor": 6
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "partitions",
    ///    "replicationFactor"
    ///  ],
    ///  "properties": {
    ///    "kafkaProperties": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "partitions": {
    ///      "type": "integer"
    ///    },
    ///    "replicationFactor": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Topic {
        #[serde(
            rename = "kafkaProperties",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub kafka_properties: std::collections::HashMap<String, String>,
        pub partitions: i64,
        #[serde(rename = "replicationFactor")]
        pub replication_factor: i64,
    }

    impl From<&Topic> for Topic {
        fn from(value: &Topic) -> Self {
            value.clone()
        }
    }

    ///TopicStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "kafkaProperties": {
    ///          "key": "kafkaProperties"
    ///        },
    ///        "partitions": 0,
    ///        "replicationFactor": 6
    ///      },
    ///      "configuration": {
    ///        "kafkaProperties": {
    ///          "key": "kafkaProperties"
    ///        },
    ///        "partitions": 0,
    ///        "replicationFactor": 6
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/Topic"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/Topic"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TopicStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<Topic>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Topic>,
        pub status: AllocationStatus,
    }

    impl From<&TopicStatus> for TopicStatus {
        fn from(value: &TopicStatus) -> Self {
            value.clone()
        }
    }

    ///Validations
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "commonName": "commonName",
    ///      "country": "country",
    ///      "locality": "locality",
    ///      "organization": "organization",
    ///      "organizationalUnit": "organizationalUnit",
    ///      "province": "province",
    ///      "subjectType": "subjectType"
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "properties": {
    ///    "commonName": {
    ///      "description": "Certificate common name",
    ///      "type": "string"
    ///    },
    ///    "country": {
    ///      "description": "Certificate country",
    ///      "type": "string"
    ///    },
    ///    "locality": {
    ///      "description": "Certificate locality",
    ///      "type": "string"
    ///    },
    ///    "organization": {
    ///      "description": "Certificate organization",
    ///      "type": "string"
    ///    },
    ///    "organizationalUnit": {
    ///      "description": "Certificate Organizational unit",
    ///      "type": "string"
    ///    },
    ///    "province": {
    ///      "description": "Certificate province",
    ///      "type": "string"
    ///    },
    ///    "subjectType": {
    ///      "description": "Certificate subject Type",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Validations {
        ///Certificate common name
        #[serde(
            rename = "commonName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub common_name: Option<String>,
        ///Certificate country
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        ///Certificate locality
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        ///Certificate organization
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub organization: Option<String>,
        ///Certificate Organizational unit
        #[serde(
            rename = "organizationalUnit",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub organizational_unit: Option<String>,
        ///Certificate province
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub province: Option<String>,
        ///Certificate subject Type
        #[serde(
            rename = "subjectType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub subject_type: Option<String>,
    }

    impl From<&Validations> for Validations {
        fn from(value: &Validations) -> Self {
            value.clone()
        }
    }

    ///Vhost
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Vhost {
        pub value: String,
    }

    impl From<&Vhost> for Vhost {
        fn from(value: &Vhost) -> Self {
            value.clone()
        }
    }

    ///Volume
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "sizeGiB": 0
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "sizeGiB"
    ///  ],
    ///  "properties": {
    ///    "sizeGiB": {
    ///      "type": "integer"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Volume {
        #[serde(rename = "sizeGiB")]
        pub size_gi_b: i64,
    }

    impl From<&Volume> for Volume {
        fn from(value: &Volume) -> Self {
            value.clone()
        }
    }

    ///VolumeStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "examples": [
    ///    {
    ///      "actual": {
    ///        "sizeGiB": 0
    ///      },
    ///      "configuration": {
    ///        "sizeGiB": 0
    ///      },
    ///      "status": {
    ///        "derivedFrom": "derivedFrom",
    ///        "notifications": [
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          },
    ///          {
    ///            "args": {
    ///              "key": "args"
    ///            },
    ///            "message": "message",
    ///            "remove": true
    ///          }
    ///        ],
    ///        "provisioned": true
    ///      }
    ///    }
    ///  ],
    ///  "type": "object",
    ///  "required": [
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "actual": {
    ///      "$ref": "#/components/schemas/Volume"
    ///    },
    ///    "configuration": {
    ///      "$ref": "#/components/schemas/Volume"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/AllocationStatus"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct VolumeStatus {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub actual: Option<Volume>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<Volume>,
        pub status: AllocationStatus,
    }

    impl From<&VolumeStatus> for VolumeStatus {
        fn from(value: &VolumeStatus) -> Self {
            value.clone()
        }
    }

    ///available networks on this platform
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "available networks on this platform",
    ///  "type": "object",
    ///  "required": [
    ///    "network"
    ///  ],
    ///  "properties": {
    ///    "network": {
    ///      "type": "string",
    ///      "enum": [
    ///        "internal",
    ///        "public"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Zone {
        pub network: ZoneNetwork,
    }

    impl From<&Zone> for Zone {
        fn from(value: &Zone) -> Self {
            value.clone()
        }
    }

    ///ZoneNetwork
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "internal",
    ///    "public"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum ZoneNetwork {
        #[serde(rename = "internal")]
        Internal,
        #[serde(rename = "public")]
        Public,
    }

    impl From<&ZoneNetwork> for ZoneNetwork {
        fn from(value: &ZoneNetwork) -> Self {
            value.clone()
        }
    }

    impl ToString for ZoneNetwork {
        fn to_string(&self) -> String {
            match *self {
                Self::Internal => "internal".to_string(),
                Self::Public => "public".to_string(),
            }
        }
    }

    impl std::str::FromStr for ZoneNetwork {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "internal" => Ok(Self::Internal),
                "public" => Ok(Self::Public),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ZoneNetwork {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ZoneNetwork {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ZoneNetwork {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn default_bool<const V: bool>() -> bool {
            V
        }

        pub(super) fn default_u64<T, const V: u64>() -> T
        where
            T: std::convert::TryFrom<u64>,
            <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
        {
            T::try_from(V).unwrap()
        }

        pub(super) fn health_check_path() -> String {
            "/".to_string()
        }

        pub(super) fn managed_tenant_services() -> Vec<super::ManagedTenantServices> {
            vec![
                super::ManagedTenantServices {
                    enabled: true,
                    name: super::ManagedTenantServicesName::Monitoring,
                },
                super::ManagedTenantServices {
                    enabled: false,
                    name: super::ManagedTenantServicesName::Vpn,
                },
                super::ManagedTenantServices {
                    enabled: false,
                    name: super::ManagedTenantServicesName::Tracing,
                },
            ]
        }

        pub(super) fn metrics_path() -> String {
            "/metrics".to_string()
        }
    }
}

#[derive(Clone, Debug)]
///Client for DSH Tenant Resource Management REST API
///
///Resource management API for DSH
///
///Version: 1.7.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.7.0"
    }
}

#[allow(clippy::all)]
impl Client {
    ///Returns the configuration of every application created by a given tenant
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/application/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_application_configuration<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<std::collections::HashMap<String, types::Application>>, Error<()>>
    {
        let url = format!(
            "{}/allocation/{}/application/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns the configuration of a certain application, specified by the
    /// tenant name and application name
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/application/{appid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    pub async fn get_allocation_by_tenant_application_by_appid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
    ) -> Result<ResponseValue<types::Application>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/application/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates an application allocation, or update it's configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/application/{appid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    /// - `body`: a JSON containing the configuration of the application you
    ///   want to deploy
    pub async fn put_allocation_by_tenant_application_by_appid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
        body: &'a types::Application,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/application/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes an application by a specified application id
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/application/{appid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    pub async fn delete_allocation_by_tenant_application_by_appid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/application/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list containing the configuration of every deployed
    /// application of a given tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/application/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_application_actual<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<std::collections::HashMap<String, types::Application>>, Error<()>>
    {
        let url = format!(
            "{}/allocation/{}/application/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of a deployed application allocation for a
    /// given app id and tenant
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/application/{appid}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    pub async fn get_allocation_by_tenant_application_by_appid_actual<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
    ) -> Result<ResponseValue<types::Application>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/application/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a status description of an application allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/application/{appid}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    pub async fn get_allocation_by_tenant_application_by_appid_status<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/application/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list containing all App Catalog App allocations and their
    /// respective configurations of a given tenant
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/appcatalogapp/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_appcatalogapp_configuration<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<std::collections::HashMap<String, types::AppCatalogApp>>, Error<()>>
    {
        let url = format!(
            "{}/allocation/{}/appcatalogapp/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list containing all App Catalog App allocations and their
    /// respective configurations of a given tenant, as they are actually
    /// deployed
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/appcatalogapp/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_appcatalogapp_actual<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<std::collections::HashMap<String, types::AppCatalogApp>>, Error<()>>
    {
        let url = format!(
            "{}/allocation/{}/appcatalogapp/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of an App Catalog App allocation by a
    /// specified tenant name and App Catalog App Id. To only view the
    /// configuration parameters of this allocation, see the
    /// `appcatalogappconfiguration` section.
    ///
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appcatalogappid`: appcatalogapp name
    pub async fn get_allocation_by_tenant_appcatalogapp_by_appcatalogappid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appcatalogappid: &'a str,
    ) -> Result<ResponseValue<types::AppCatalogApp>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/appcatalogapp/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appcatalogappid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of an App Catalog App allocation as it is
    /// actually deployed. To only view the configuration parameters of this
    /// allocation, see the `appcatalogappconfiguration` section.
    ///
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/appcatalogapp/{appcatalogappid}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appcatalogappid`: appcatalogapp name
    pub async fn get_allocation_by_tenant_appcatalogapp_by_appcatalogappid_actual<'a>(
        &'a self,
        tenant: &'a str,
        appcatalogappid: &'a str,
    ) -> Result<ResponseValue<types::AppCatalogApp>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/appcatalogapp/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appcatalogappid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns the wanted configuration of an App Catalog App by its tenant
    /// name and AppCatalogApp Id. If an App Catalog App is stuck while
    /// deploying and not on actual, it will show up here
    ///
    ///Sends a `GET` request to
    /// `/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appcatalogappid`: appcatalogapp name
    pub async fn get_appcatalog_by_tenant_appcatalogapp_by_appcatalogappid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appcatalogappid: &'a str,
    ) -> Result<ResponseValue<types::AppCatalogAppConfiguration>, Error<()>> {
        let url = format!(
            "{}/appcatalog/{}/appcatalogapp/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appcatalogappid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates a new App Catalog App, or update its configuration
    ///
    ///Sends a `PUT` request to
    /// `/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appcatalogappid`: appcatalogapp name
    /// - `body`: JSON object containing required parameters for AppCatalogApp
    ///   manifest. This is comparable to the configuration object on a regular
    ///   Application service.
    pub async fn put_appcatalog_by_tenant_appcatalogapp_by_appcatalogappid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appcatalogappid: &'a str,
        body: &'a types::AppCatalogAppConfiguration,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/appcatalog/{}/appcatalogapp/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appcatalogappid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes an App Catalog App
    ///
    ///Sends a `DELETE` request to
    /// `/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appcatalogappid`: appcatalogapp name
    pub async fn delete_appcatalog_by_tenant_appcatalogapp_by_appcatalogappid_configuration<'a>(
        &'a self,
        tenant: &'a str,
        appcatalogappid: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/appcatalog/{}/appcatalogapp/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appcatalogappid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets status description of an App Catalog App
    ///
    ///Sends a `GET` request to
    /// `/appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appcatalogappid`: appcatalogapp name
    pub async fn get_appcatalog_by_tenant_appcatalogapp_by_appcatalogappid_status<'a>(
        &'a self,
        tenant: &'a str,
        appcatalogappid: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/appcatalog/{}/appcatalogapp/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appcatalogappid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list of AppCatalog manifests for a given tenant
    ///
    ///Sends a `GET` request to `/appcatalog/{tenant}/manifest`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_appcatalog_by_tenant_manifest<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<Vec<types::AppCatalogManifest>>, Error<()>> {
        let url = format!(
            "{}/appcatalog/{}/manifest",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///lists all bucketwatches of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucketwatch`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_bucketwatch<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucketwatch",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///shows overall status of a bucketwatch allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket/{id}/bucketwatch`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketwatch<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::BucketWatchStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketwatch",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets configuration of a bucketwatch allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketwatch/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketwatch_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::BucketWatch>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketwatch/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates bucketwatch configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketwatch/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn put_allocation_by_tenant_bucket_by_id_bucketwatch_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketwatch/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a bucketwatch
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketwatch/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn delete_allocation_by_tenant_bucket_by_id_bucketwatch_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketwatch/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets actual configuration of a bucketwatch allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketwatch/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketwatch_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::BucketWatch>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketwatch/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets status description of a bucketwatch allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketwatch/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketwatch_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketwatch/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///lists all bucket names of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_bucket<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///shows overall status of a bucket allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::BucketStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets configuration of a bucket allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Bucket>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates bucket configuration.It is impossible to update an existing
    /// bucket. This requires a delete of the existing bucket and creation of a
    /// new one with the wanted configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/bucket/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `body`: the JSON representation of the resource
    pub async fn put_allocation_by_tenant_bucket_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: &'a types::Bucket,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a bucket
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/bucket/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn delete_allocation_by_tenant_bucket_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets actual configuration of a bucket allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Bucket>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets status description of a bucket allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///lists all bucketaccesses of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucketaccess`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_bucketaccess<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucketaccess",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///shows bucketaccesses about a specific bucket
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket/{id}/bucketaccess`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketaccess<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///shows overall status of a third party bucket
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketaccess/{name}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `name`: bucket access name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketaccess_by_name<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<types::BucketAccessStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets configuration of a bucketaccess allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `name`: bucket access name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketaccess_by_name_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<types::BucketAccessConfiguration>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates bucketaccess configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `name`: bucket access name
    /// - `body`: the wanted config of the (new) bucketaccess allocation
    pub async fn put_allocation_by_tenant_bucket_by_id_bucketaccess_by_name_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        name: &'a str,
        body: &'a types::BucketAccessConfiguration,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a bucketaccess
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `name`: bucket access name
    pub async fn delete_allocation_by_tenant_bucket_by_id_bucketaccess_by_name_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets actual configuration of a bucketaccess allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `name`: bucket access name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketaccess_by_name_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<types::BucketAccess>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets status description of a bucketaccess allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/bucket/{id}/bucketaccess/{name}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: bucket name
    /// - `name`: bucket access name
    pub async fn get_allocation_by_tenant_bucket_by_id_bucketaccess_by_name_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/{}/bucketaccess/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list of all certificate names that are allocated to a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/certificate`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_certificate<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the status of a specific certificate allocation by id
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/certificate/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: certificate name
    pub async fn get_allocation_by_tenant_certificate_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::CertificateStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of a certificate allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/certificate/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: certificate name
    pub async fn get_allocation_by_tenant_certificate_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Certificate>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create a new certificate. It is impossible to update an existing
    /// certificate. This requires a delete of the existing certificate and
    /// creation of a new one with the wanted configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/certificate/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: certificate name
    /// - `body`: the JSON object containing the configuration of a certificate.
    ///   certChainSecret and keySecret must be known to the platform.
    pub async fn put_allocation_by_tenant_certificate_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: &'a types::Certificate,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a certificate by id
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/certificate/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: certificate name
    pub async fn delete_allocation_by_tenant_certificate_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the actual configuration of a certificate allocation. This may
    /// not represent the wanted configuration
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/certificate/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: certificate name
    pub async fn get_allocation_by_tenant_certificate_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Certificate>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a certificate allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/certificate/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: certificate name
    pub async fn get_allocation_by_tenant_certificate_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/certificate/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) lists ids of all databases of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/database`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_database<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) gets overall status of a database allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/database/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: database name
    pub async fn get_allocation_by_tenant_database_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::DatabaseStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) gets configuration for a database allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/database/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: database name
    pub async fn get_allocation_by_tenant_database_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Database>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) creates a database configuration. It is impossible to
    /// update an existing database
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/database/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: database name
    /// - `body`: the JSON representation of the resource
    pub async fn put_allocation_by_tenant_database_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: &'a types::Database,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) deletes a database
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/database/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: database name
    pub async fn delete_allocation_by_tenant_database_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) gets actual state for a database allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/database/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: database name
    pub async fn get_allocation_by_tenant_database_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Database>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///(beta release) gets status description of a database allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/database/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: database name
    pub async fn get_allocation_by_tenant_database_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/database/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///lists all data catalog assets of a tenant for the given kind
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/datacatalog/asset/{kind}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `kind`: data catalog asset kind
    pub async fn get_allocation_by_tenant_datacatalog_asset_by_kind<'a>(
        &'a self,
        tenant: &'a str,
        kind: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/datacatalog/asset/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///shows overall status of a datacatalog asset allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/datacatalog/asset/{kind}/{name}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `kind`: data catalog asset kind
    /// - `name`: data catalog asset name
    pub async fn get_allocation_by_tenant_datacatalog_asset_by_kind_by_name<'a>(
        &'a self,
        tenant: &'a str,
        kind: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<types::DataCatalogAssetStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/datacatalog/asset/{}/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets configuration of a data catalog asset allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `kind`: data catalog asset kind
    /// - `name`: data catalog asset name
    pub async fn get_allocation_by_tenant_datacatalog_asset_by_kind_by_name_configuration<'a>(
        &'a self,
        tenant: &'a str,
        kind: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<types::DataCatalogAsset>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/datacatalog/asset/{}/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates dataCatalogAsset configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `kind`: data catalog asset kind
    /// - `name`: data catalog asset name
    /// - `body`: the JSON representation of the resource
    pub async fn put_allocation_by_tenant_datacatalog_asset_by_kind_by_name_configuration<'a>(
        &'a self,
        tenant: &'a str,
        kind: &'a str,
        name: &'a str,
        body: &'a types::DataCatalogAsset,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/datacatalog/asset/{}/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a dataCatalogAsset
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/datacatalog/asset/{kind}/{name}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `kind`: data catalog asset kind
    /// - `name`: data catalog asset name
    pub async fn delete_allocation_by_tenant_datacatalog_asset_by_kind_by_name_configuration<'a>(
        &'a self,
        tenant: &'a str,
        kind: &'a str,
        name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/datacatalog/asset/{}/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
            encode_path(&name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the overall status of a Flink Cluster
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/flinkcluster`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_flinkcluster<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::FlinkClusterStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/flinkcluster",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of a Flink Cluster
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/flinkcluster/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_flinkcluster_configuration<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::FlinkCluster>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/flinkcluster/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create a new Flink Cluster. It is impossible to update an existing Flink
    /// Cluster. This requires a delete of the existing Flink Cluster and
    /// creation of a new one with the wanted configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/flinkcluster/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `body`: a JSON object containing the desired configuration of the
    ///   Flink Cluster. Zone must be known to the platform.
    pub async fn put_allocation_by_tenant_flinkcluster_configuration<'a>(
        &'a self,
        tenant: &'a str,
        body: &'a types::FlinkCluster,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/flinkcluster/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a Flink Cluster. Since only one cluster can be created per
    /// tenant, only the tenants' name needs to be specified
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/flinkcluster/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn delete_allocation_by_tenant_flinkcluster_configuration<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/flinkcluster/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the actual configuration of a Flink Cluster
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/flinkcluster/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_flinkcluster_actual<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::FlinkCluster>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/flinkcluster/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a Flink Cluster
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/flinkcluster/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_flinkcluster_status<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/flinkcluster/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list of all kafka proxies of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/kafkaproxy`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_kafkaproxy<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/kafkaproxy",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns the configuration of a certain kafka Proxy, specified by the
    /// tenant name and kafka Proxy name
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/kafkaproxy/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: Kafka proxy id
    pub async fn get_allocation_by_tenant_kafkaproxy_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::KafkaProxy>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/kafkaproxy/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update the value of the kafka proxy
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/kafkaproxy/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: Kafka proxy id
    /// - `body`: the kafka proxy configuration options
    pub async fn put_allocation_by_tenant_kafkaproxy_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: &'a types::KafkaProxy,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/kafkaproxy/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a kafka proxy
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/kafkaproxy/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: Kafka proxy id
    pub async fn delete_allocation_by_tenant_kafkaproxy_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/kafkaproxy/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///generate new client secret for a tenant
    ///
    ///Sends a `GET` request to `/robot/{tenant}/generate-secret`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_robot_by_tenant_generate_secret<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ClientSecret>, Error<()>> {
        let url = format!(
            "{}/robot/{}/generate-secret",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list of all secret names of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/secret`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_secret<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create a new secret
    ///
    ///Sends a `POST` request to `/allocation/{tenant}/secret`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `body`: a JSON object containing the name and the secret value
    pub async fn post_allocation_by_tenant_secret<'a>(
        &'a self,
        tenant: &'a str,
        body: &'a types::Secret,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of a secret allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/secret/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: secret name
    pub async fn get_allocation_by_tenant_secret_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Empty>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a secret
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/secret/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: secret name
    pub async fn delete_allocation_by_tenant_secret_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the actual state of a secret. The response body will always be
    /// empty because we cannot share the secret value, but the response code
    /// will tell you more about its state
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/secret/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: secret name
    pub async fn get_allocation_by_tenant_secret_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Empty>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a secret allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/secret/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: secret name
    pub async fn get_allocation_by_tenant_secret_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the value of a secret
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/secret/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: secret name
    pub async fn get_allocation_by_tenant_secret_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update the value of a secret
    ///
    ///Sends a `PUT` request to `/allocation/{tenant}/secret/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: secret name
    /// - `body`: the secret value as a string
    pub async fn put_allocation_by_tenant_secret_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: String,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/secret/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("text/plain"),
            )
            .body(body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///return a list containing the ids of all applications with derived tasks
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/task`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_task<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/task",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///return a list containing the ids of an application's derived tasks
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/task/{appid}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    pub async fn get_allocation_by_tenant_task_by_appid<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/task/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns overall status of a task
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/task/{appid}/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    /// - `id`: task name
    pub async fn get_allocation_by_tenant_task_by_appid_by_id<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::TaskStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/task/{}/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the actual state of a specific task
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/task/{appid}/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    /// - `id`: task name
    pub async fn get_allocation_by_tenant_task_by_appid_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Task>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/task/{}/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a task
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/task/{appid}/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `appid`: application name
    /// - `id`: task name
    pub async fn get_allocation_by_tenant_task_by_appid_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        appid: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/task/{}/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&appid.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///lists only bucket names of a tenant that originated from a third party
    /// bucket
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/bucket/fromthirdparty`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_bucket_fromthirdparty<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/bucket/fromthirdparty",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///list summaries of third party buckets, registered using credentials
    /// shared to you by a third party
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_thirdpartybucketconcession<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///register a new bucket concession for which credentials were shared to
    /// you by a third party
    ///
    ///Sends a `POST` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `body`: the secret value
    pub async fn post_allocation_by_tenant_thirdpartybucketconcession<'a>(
        &'a self,
        tenant: &'a str,
        body: &'a types::ThirdPartyBucketConcessionRegistration,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///shows overall status of a third party bucket
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: your name of choice for the third party bucket
    pub async fn get_allocation_by_tenant_thirdpartybucketconcession_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ThirdPartyBucketConcessionStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets configuration of a third party bucket (received bucket access)
    /// allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: your name of choice for the third party bucket
    pub async fn get_allocation_by_tenant_thirdpartybucketconcession_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ThirdPartyBucketConcession>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///unregisters a third party bucket. This will also remove the virtual
    /// bucket
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: your name of choice for the third party bucket
    pub async fn delete_allocation_by_tenant_thirdpartybucketconcession_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets actual configuration of a third party bucket (received bucket
    /// access) allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: your name of choice for the third party bucket
    pub async fn get_allocation_by_tenant_thirdpartybucketconcession_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ThirdPartyBucketConcession>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///gets status description of third party bucket (received bucket access)
    /// allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/thirdpartybucketconcession/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: your name of choice for the third party bucket
    pub async fn get_allocation_by_tenant_thirdpartybucketconcession_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/thirdpartybucketconcession/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list of topics of a tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/topic`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_topic<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the overall status of a topic allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/topic/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: topic name
    pub async fn get_allocation_by_tenant_topic_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::TopicStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of a topic allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/topic/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: topic name
    pub async fn get_allocation_by_tenant_topic_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Topic>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create a new topic. It is impossible to update an existing topic. This
    /// requires a delete of the existing topic and creation of a new one with
    /// the wanted configuration
    ///
    ///Sends a `PUT` request to `/allocation/{tenant}/topic/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: topic name
    /// - `body`: the JSON object containing the configuration of the desired
    ///   topic
    pub async fn put_allocation_by_tenant_topic_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: &'a types::Topic,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a topic
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/topic/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: topic name
    pub async fn delete_allocation_by_tenant_topic_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns actual configuration of a topic allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/topic/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: topic name
    pub async fn get_allocation_by_tenant_topic_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Topic>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a topic allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/topic/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: topic name
    pub async fn get_allocation_by_tenant_topic_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/topic/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list containing the ids of all volumes of a given tenant
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/volume`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    pub async fn get_allocation_by_tenant_volume<'a>(
        &'a self,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume",
            self.baseurl,
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the overall status of a volume allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/volume/{id}`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: volume name
    pub async fn get_allocation_by_tenant_volume_by_id<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::VolumeStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume/{}",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration for a volume allocation
    ///
    ///Sends a `GET` request to
    /// `/allocation/{tenant}/volume/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: volume name
    pub async fn get_allocation_by_tenant_volume_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Volume>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create a new volume configuration. It is impossible to update an
    /// existing volume. This requires a delete of the existing volume and
    /// creation of a new one with the wanted configuration
    ///
    ///Sends a `PUT` request to
    /// `/allocation/{tenant}/volume/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: volume name
    /// - `body`: the JSON object containing the desired configuration of a
    ///   volume allocation
    pub async fn put_allocation_by_tenant_volume_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
        body: &'a types::Volume,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a volume
    ///
    ///Sends a `DELETE` request to
    /// `/allocation/{tenant}/volume/{id}/configuration`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: volume name
    pub async fn delete_allocation_by_tenant_volume_by_id_configuration<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume/{}/configuration",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the actual state for a volume allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/volume/{id}/actual`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: volume name
    pub async fn get_allocation_by_tenant_volume_by_id_actual<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::Volume>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume/{}/actual",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a volume allocation
    ///
    ///Sends a `GET` request to `/allocation/{tenant}/volume/{id}/status`
    ///
    ///Arguments:
    /// - `tenant`: tenant name
    /// - `id`: volume name
    pub async fn get_allocation_by_tenant_volume_by_id_status<'a>(
        &'a self,
        tenant: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/allocation/{}/volume/{}/status",
            self.baseurl,
            encode_path(&tenant.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a list of tenants managed by the `manager` tenant
    ///
    ///Sends a `GET` request to `/manage/{manager}/tenant`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    pub async fn get_manage_by_manager_tenant<'a>(
        &'a self,
        manager: &'a str,
    ) -> Result<ResponseValue<types::ChildList>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant",
            self.baseurl,
            encode_path(&manager.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the configuration of tenant as managed by the manager
    ///
    ///Sends a `GET` request to
    /// `/manage/{manager}/tenant/{tenant}/configuration`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    pub async fn get_manage_by_manager_tenant_by_tenant_configuration<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ManagedTenant>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/configuration",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///creates and/or updates a managed tenant for managing tenant or update
    /// its configuration
    ///
    ///Sends a `PUT` request to
    /// `/manage/{manager}/tenant/{tenant}/configuration`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    /// - `body`: the JSON object containing the configuration of the managed
    ///   tenant
    pub async fn put_manage_by_manager_tenant_by_tenant_configuration<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
        body: &'a types::ManagedTenant,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/configuration",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///deletes a managed tenant for the managing tenant
    ///
    ///Sends a `DELETE` request to
    /// `/manage/{manager}/tenant/{tenant}/configuration`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    pub async fn delete_manage_by_manager_tenant_by_tenant_configuration<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/configuration",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns the actual state for a managed tenant allocation
    ///
    ///Sends a `GET` request to `/manage/{manager}/tenant/{tenant}/actual`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    pub async fn get_manage_by_manager_tenant_by_tenant_actual<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::ManagedTenant>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/actual",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///returns a brief status description of a managed tenant allocation
    ///
    ///Sends a `GET` request to `/manage/{manager}/tenant/{tenant}/status`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    pub async fn get_manage_by_manager_tenant_by_tenant_status<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
    ) -> Result<ResponseValue<types::AllocationStatus>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/status",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///get all limits of a managed tenant
    ///
    ///Sends a `GET` request to `/manage/{manager}/tenant/{tenant}/limit`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    pub async fn get_manage_by_manager_tenant_by_tenant_limit<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
    ) -> Result<ResponseValue<Vec<types::LimitValue>>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/limit",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///update multiple limits of a managed tenant
    ///
    ///Sends a `PATCH` request to `/manage/{manager}/tenant/{tenant}/limit`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    /// - `body`: a JSON list with multiple limits of the managed tenant
    pub async fn patch_manage_by_manager_tenant_by_tenant_limit<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
        body: &'a Vec<types::LimitValue>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/limit",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.patch(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///get a specific managed tenant limit set by the managing tenant
    ///
    ///Sends a `GET` request to
    /// `/manage/{manager}/tenant/{tenant}/limit/{kind}`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    /// - `kind`: Limit request type
    pub async fn get_manage_by_manager_tenant_by_tenant_limit_by_kind<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
        kind: types::GetManageByManagerTenantByTenantLimitByKindKind,
    ) -> Result<ResponseValue<types::LimitValue>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/limit/{}",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create and/or update the configured limits for a managed tenant
    ///
    ///Sends a `PUT` request to
    /// `/manage/{manager}/tenant/{tenant}/limit/{kind}`
    ///
    ///Arguments:
    /// - `manager`: Name of the tenant that is acting as manager for this
    ///   request
    /// - `tenant`: tenant name
    /// - `kind`: Limit request type
    /// - `body`: the JSON object containing the limit configuration of the
    ///   managed tenant
    pub async fn put_manage_by_manager_tenant_by_tenant_limit_by_kind<'a>(
        &'a self,
        manager: &'a str,
        tenant: &'a str,
        kind: types::PutManageByManagerTenantByTenantLimitByKindKind,
        body: &'a types::LimitValue,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/manage/{}/tenant/{}/limit/{}",
            self.baseurl,
            encode_path(&manager.to_string()),
            encode_path(&tenant.to_string()),
            encode_path(&kind.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            202u16 => Ok(ResponseValue::empty(response)),
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
