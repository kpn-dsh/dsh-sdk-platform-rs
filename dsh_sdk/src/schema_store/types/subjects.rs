use serde::{Deserialize, Serialize};

use super::SchemaType;

/// Subject version
///
/// Select a specific `version` of the subject or the `latest` version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_version_from_string() {
        let latest: SubjectVersion = "latest".into();
        assert_eq!(latest, SubjectVersion::Latest);

        let version: SubjectVersion = "1".into();
        assert_eq!(version, SubjectVersion::Version(1));

        let version: SubjectVersion = "2".into();
        assert_eq!(version, SubjectVersion::Version(2));
    }

    #[test]
    fn test_subject_version_from_i32() {
        let version: SubjectVersion = 1.into();
        assert_eq!(version, SubjectVersion::Version(1));

        let version: SubjectVersion = 2.into();
        assert_eq!(version, SubjectVersion::Version(2));
    }

    #[test]
    fn test_subject_version_display() {
        let latest = SubjectVersion::Latest;
        assert_eq!(latest.to_string(), "latest");

        let version = SubjectVersion::Version(1);
        assert_eq!(version.to_string(), "1");
    }

    #[test]
    fn test_subject_version_default() {
        let default = SubjectVersion::default();
        assert_eq!(default, SubjectVersion::Latest);
    }

    #[test]
    fn test_subject_version_from_string_default() {
        let default: SubjectVersion = "invalid".into();
        assert_eq!(default, SubjectVersion::Latest);
    }

    #[test]
    fn test_subject_serde() {
        let subject_proto =
            r#"{"subject":"test","id":1,"version":1,"schemaType":"PROTOBUF","schema":"schema"}"#;
        let subject: Subject = serde_json::from_str(subject_proto).unwrap();
        assert_eq!(subject.subject, "test");
        assert_eq!(subject.id, 1);
        assert_eq!(subject.version, 1);
        assert_eq!(subject.schema_type, SchemaType::PROTOBUF);
        assert_eq!(subject.schema, "schema");

        let subject_json =
            r#"{"subject":"test","id":1,"version":1,"schemaType":"JSON","schema":"schema"}"#;
        let subject: Subject = serde_json::from_str(subject_json).unwrap();
        assert_eq!(subject.subject, "test");
        assert_eq!(subject.id, 1);
        assert_eq!(subject.version, 1);
        assert_eq!(subject.schema_type, SchemaType::JSON);
        assert_eq!(subject.schema, "schema");

        let subject_avro = r#"{"subject":"test","id":1,"version":1,"schema":"schema"}"#;
        let subject: Subject = serde_json::from_str(subject_avro).unwrap();
        assert_eq!(subject.subject, "test");
        assert_eq!(subject.id, 1);
        assert_eq!(subject.version, 1);
        assert_eq!(subject.schema_type, SchemaType::AVRO);
        assert_eq!(subject.schema, "schema");
    }
}
