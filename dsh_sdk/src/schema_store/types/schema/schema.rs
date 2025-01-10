use std::io::Cursor;

use apache_avro::Schema as AvroSchema;
use protofish::context::Context as ProtoSchema;
use serde_json::Value as JsonValue;

use crate::schema_store::error::SchemaStoreError;

/// Schema object
///
/// Common object to apply schema operations on
enum SchemaObject {
    Avro(AvroSchema),
    Json(JsonValue),
    Proto(ProtoSchema),
}

impl SchemaObject {
    /// Deserialize bytes into struct
    /// 
    /// Bytes should only contain the encoded data and not the magic bytes
    pub fn decode<T>(&self, bytes: &[u8]) -> Result<T, SchemaStoreError> 
    where T: serde::de::DeserializeOwned {
        match self {
            Self::Avro(schema) => {
                apache_avro::from_value(&to_avro_value(schema, bytes)?).map_err(|_| SchemaStoreError::FailedParseToStruct)
            }
            Self::Json(schema) => serde_json::from_slice(bytes).map_err(SchemaStoreError::from),
            Self::Proto(_) => Err(SchemaStoreError::NotImplementedProtobufDeserialize),
        }
    }
}




fn to_avro_value(schema: &AvroSchema, bytes: &[u8]) -> Result<apache_avro::types::Value, SchemaStoreError> {
    let mut buf = Cursor::new(bytes);
    let value= apache_avro::from_avro_datum(schema, & mut buf, None).map_err(|e| {
        log::warn!("Failed to decode value for Avro schema {}: {}", schema.name().map(|n|n.name.clone()).unwrap_or_default() , e);
        SchemaStoreError::FailedToDecode(e.to_string())
    })?;
    Ok(value)
}