use super::SchemaType;
use crate::schema_store::SchemaStoreError;
use serde::{Deserialize, Serialize};

/// Structure to post (new) schema to a (new) subject or verify if a schema already exists for a subject
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawSchemaWithType {
    #[serde(default)]
    pub schema_type: SchemaType,
    pub schema: String,
}

impl RawSchemaWithType {
    /// Create [RawSchemaWithType] from a schema string.
    ///
    /// It returns error if it cannot parse it into to a Avro, JSON or Protobuf schema.
    pub fn parse<S>(schema: S) -> Result<Self, SchemaStoreError>
    where
        S: AsRef<str>,
    {
        schema.as_ref().try_into()
    }

    /// Raw schema string
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// Schema type (AVRO, JSON, PROTOBUF)
    pub fn schema_type(&self) -> SchemaType {
        self.schema_type
    }
}

impl From<super::Subject> for RawSchemaWithType {
    fn from(value: super::Subject) -> Self {
        Self {
            schema_type: value.schema_type,
            schema: value.schema,
        }
    }
}

impl TryFrom<String> for RawSchemaWithType {
    type Error = SchemaStoreError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&str> for RawSchemaWithType {
    type Error = SchemaStoreError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(avro) = apache_avro::Schema::parse_str(value) {
            log::debug!("avro: {}", avro.canonical_form());
            Ok(Self {
                schema_type: SchemaType::AVRO,
                schema: avro.canonical_form(),
            })
        } else if let Ok(json) = serde_json::from_str::<serde_json::Value>(value) {
            log::debug!("json: {:?}", json);
            Ok(Self {
                schema_type: SchemaType::JSON,
                schema: json.to_string(),
            })
        } else if let Ok(_) = protofish::context::Context::parse(&[value]) {
            // TODO: Add parser for protobuf
            log::debug!("protobuf: {:?}", value);
            Ok(Self {
                schema_type: SchemaType::PROTOBUF,
                schema: value.to_string(),
            })
        } else {
            Err(SchemaStoreError::FailedToParseSchema(None))
        }
    }
}

impl TryFrom<apache_avro::Schema> for RawSchemaWithType {type Error = SchemaStoreError;

    fn try_from(value: apache_avro::Schema) -> Result<Self, Self::Error> {
        Ok(Self {
            schema_type: SchemaType::AVRO,
            schema: value.canonical_form(),
        })
    }
}

impl TryFrom<serde_json::Value> for RawSchemaWithType {
    type Error = SchemaStoreError;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error>  {
        Ok(if let Ok(avro) = apache_avro::Schema::parse(&value) {
            Self {
                schema_type: SchemaType::AVRO,
                schema: avro.canonical_form(),
            }
        } else {
            Self {
                schema_type: SchemaType::JSON,
                schema: value.to_string(),
            }
        })
    }
}

impl<S> TryFrom<(S, SchemaType)> for RawSchemaWithType
where
    S: AsRef<str>,
{
    type Error = SchemaStoreError;

    fn try_from(value: (S, SchemaType)) -> Result<Self, Self::Error> {
        let schema_type = value.1;
        let raw_schema = value.0.as_ref();
        let raw_schema = match schema_type {
            SchemaType::JSON => {
                let _ = serde_json::from_str::<serde_json::Value>(raw_schema).map_err(|_| SchemaStoreError::FailedToParseSchema(Some(schema_type)))?;
                raw_schema
            }
            SchemaType::AVRO => {
                let _ = apache_avro::Schema::parse_str(raw_schema).map_err(|_| SchemaStoreError::FailedToParseSchema(Some(schema_type)))?;
                raw_schema
            }
            SchemaType::PROTOBUF => {
                let _ = protofish::context::Context::parse(&[raw_schema]).map_err(|_| SchemaStoreError::FailedToParseSchema(Some(schema_type)))?;
                raw_schema
            }
        };
        Ok(Self {
            schema_type,
            schema: raw_schema.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apache_avro::Schema as AvroSchema;
    use serde_json::Value as JsonValue;

    #[test]
    fn test_parse_avro() {
        let raw_schema = r#"{"name":"User","type":"record","fields":[{"name":"name","type":"string"}]}"#;
        let schema = RawSchemaWithType::parse(raw_schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::AVRO);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_parse_json() {
        let raw_schema = r#"{"fields":[{"name":"name","type":"string"}],"name":"User"}"#;
        let schema = RawSchemaWithType::parse(raw_schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::JSON);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_parse_protobuf() {
        let raw_schema = r#"syntax = "proto3"; message User { string name = 1; }"#;
        let schema = RawSchemaWithType::parse(raw_schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::PROTOBUF);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_parse_invalid() {
        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}"#;
        let schema = RawSchemaWithType::parse(raw_schema);
        assert!(schema.is_err());

        let raw_schema = r#"not a schema"#;
        let schema = RawSchemaWithType::parse(raw_schema);
        assert!(schema.is_err());
    }

    #[test]
    fn test_try_from_avro() {
        let raw_schema = r#"{"name":"User","type":"record","fields":[{"name":"name","type":"string"}]}"#;
        let schema = apache_avro::Schema::parse_str(raw_schema).unwrap();
        let schema = RawSchemaWithType::try_from(schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::AVRO);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_json() {
        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}]}"#;
        let schema = serde_json::from_str::<JsonValue>(raw_schema).unwrap();
        let schema = RawSchemaWithType::try_from(schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::JSON);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_protobuf() {
        let raw_schema = r#"syntax = "proto3"; message User { string name = 1; }"#;
        let schema = RawSchemaWithType::try_from(raw_schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::PROTOBUF);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_tuple_json() {
        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}]}"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::JSON)).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::JSON);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_tuple_avro() {
        let raw_schema = r#"{"name":"User","type":"record","fields":[{"name":"name","type":"string"}]}"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::AVRO)).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::AVRO);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_tuple_protobuf() {
        let raw_schema = r#"syntax = "proto3"; message User { string name = 1; }"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::PROTOBUF)).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::PROTOBUF);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_tuple_invalid() {
        let raw_schema = r#"syntax = "proto3"; message User { string name = 1; }"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::JSON));
        assert!(schema.is_err());

        let raw_schema = r#"name":"User","fields":[{"name":"name","type":"string"}]}"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::AVRO));
        assert!(schema.is_err());

        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::PROTOBUF));
        assert!(schema.is_err());

        let raw_schema = r#"not a schema"#;
        let schema = RawSchemaWithType::try_from((raw_schema, SchemaType::PROTOBUF));
        assert!(schema.is_err());
    }

    #[test]
    fn test_try_from_string() {
        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}]}"#;
        let schema = RawSchemaWithType::try_from(raw_schema.to_string()).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::JSON);
        assert_eq!(schema.schema(), raw_schema);

        let raw_schema = r#"{"name":"User","type":"record","fields":[{"name":"name","type":"string"}]}"#;
        let schema = RawSchemaWithType::try_from(raw_schema.to_string()).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::AVRO);
        assert_eq!(schema.schema(), raw_schema);

        let raw_schema = r#"syntax = "proto3"; message User { string name = 1; }"#;
        let schema = RawSchemaWithType::try_from(raw_schema.to_string()).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::PROTOBUF);
        assert_eq!(schema.schema(), raw_schema);

        let raw_schema = r#"not a schema"#;
        let schema = RawSchemaWithType::try_from(raw_schema.to_string());
        assert!(schema.is_err());
    }


    #[test]
    fn test_try_from_string_invalid() {
        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}"#;
        let schema = RawSchemaWithType::try_from(raw_schema.to_string());
        assert!(schema.is_err());

        let raw_schema = r#"not a schema"#;
        let schema = RawSchemaWithType::try_from(raw_schema.to_string());
        assert!(schema.is_err());
    }

    #[test]
    fn test_from_subject() {
        let raw_schema = r#"{"name":"User","type":"record","fields":[{"name":"name","type":"string"}]}"#;
        let subject = crate::schema_store::types::Subject {
            version: 1,
            id: 1,
            subject: "test".to_string(),
            schema_type: SchemaType::AVRO,
            schema: raw_schema.to_string(),
        };
        let schema = RawSchemaWithType::from(subject);
        assert_eq!(schema.schema_type(), SchemaType::AVRO);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_json_value() {
        let raw_schema = r#"{"name":"User","fields":[{"name":"name","type":"string"}]}"#;
        let schema = serde_json::from_str::<JsonValue>(raw_schema).unwrap();
        let schema = RawSchemaWithType::try_from(schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::JSON);
        assert_eq!(schema.schema(), raw_schema);
    }

    #[test]
    fn test_try_from_json_value_avro() {
        let raw_schema = r#"{"name":"User","type":"record","fields":[{"name":"name","type":"string"}]}"#;
        let schema = AvroSchema::parse_str(raw_schema).unwrap();
        let schema = RawSchemaWithType::try_from(schema).unwrap();
        assert_eq!(schema.schema_type(), SchemaType::AVRO);
        assert_eq!(schema.schema(), raw_schema);
    }
}