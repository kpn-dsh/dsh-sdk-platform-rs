use serde::{Deserialize, Serialize};

/// Schema id
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaId {
    pub id: i32,
}

impl SchemaId {
    pub fn id(&self) -> i32 {
        self.id
    }
}
impl From<i32> for SchemaId {
    fn from(value: i32) -> Self {
        Self { id: value }
    }
}

impl From<SchemaId> for i32 {
    fn from(value: SchemaId) -> Self {
        value.id
    }
}
