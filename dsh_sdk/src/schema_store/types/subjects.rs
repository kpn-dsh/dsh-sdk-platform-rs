use serde::{Deserialize, Serialize};

use super::SchemaType;

/// Subject version
///
/// Select a specific `version` of the subject or the `latest` version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectVersion {
    Latest,
    Version(i32),
}

/// Subject
///
/// All related info related subject
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub subject: String,
    pub id: i32,
    pub version: i32,
    #[serde(default)]
    pub schema_type: SchemaType,
    pub schema: String,
}

/// Subject version
///
/// Subjects related version
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectVersionInfo {
    pub subject: String,
    pub version: i32,
}

impl Default for SubjectVersion {
    fn default() -> Self {
        Self::Latest
    }
}

impl From<String> for SubjectVersion {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for SubjectVersion {
    fn from(value: &str) -> Self {
        match value {
            "latest" => Self::Latest,
            version => match version.parse::<i32>() {
                Ok(version) => Self::Version(version),
                Err(_) => Self::Latest,
            },
        }
    }
}

impl From<i32> for SubjectVersion {
    fn from(value: i32) -> Self {
        Self::Version(value)
    }
}

impl std::fmt::Display for SubjectVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Latest => write!(f, "latest"),
            Self::Version(version) => write!(f, "{}", version),
        }
    }
}
