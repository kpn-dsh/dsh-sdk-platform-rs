use serde::{Deserialize, Serialize};

/// Schema type
///
/// Available schema types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SchemaType {
    JSON,
    PROTOBUF,
    AVRO,
}

impl Default for SchemaType {
    fn default() -> Self {
        Self::AVRO
    }
}

impl<S> From<S> for SchemaType
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        match value.as_ref() {
            "JSON" | "json" => Self::JSON,
            "PROTOBUF" | "protobuf" => Self::PROTOBUF,
            "AVRO" | "avro" => Self::AVRO,
            _ => Self::AVRO,
        }
    }
}

impl std::fmt::Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JSON => write!(f, "JSON"),
            Self::PROTOBUF => write!(f, "PROTOBUF"),
            Self::AVRO => write!(f, "AVRO"),
        }
    }
}
