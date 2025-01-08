use super::api::SchemaStoreApi;
use super::request::Request;
use super::types::*;
use super::Result;
use crate::Dsh;

/// High level Schema Store Client
///
/// Client to interact with the Schema Store API.
pub struct SchemaStoreClient<C: Request> {
    pub(crate) base_url: String,
    pub(crate) client: C,
}

impl SchemaStoreClient<reqwest::Client> {
    pub fn new() -> Self {
        Self::new_with_base_url(Dsh::get().schema_registry_host())
    }

    /// Create SchemaStoreClient with a custom base URL
    pub fn new_with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Request::new_client(),
        }
    }
}

impl<C> SchemaStoreClient<C>
where
    C: Request,
{
    /// Get the compatibility level for a subject
    ///
    /// ## Returns
    /// Returns a Result of the compatibility level of given subject
    ///
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::SubjectName;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    /// println!("Config: {:?}", client.subject_compatibility(&subject_name).await);
    /// # Ok(())
    /// # }
    ///
    pub async fn subject_compatibility(&self, subject: &SubjectName) -> Result<Compatibility> {
        Ok(self.get_config_subject(subject.name()).await?.into())
    }

    /// Set the compatibility level for a subject
    ///
    /// Set compatibility on subject level. With 1 schema stored in the subject, you can change it to any compatibility level.
    /// Else, you can only change into a less restrictive level.
    ///
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    ///
    /// ## Returns
    /// Returns a Result of the new compatibility level
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::{Compatibility, SubjectName};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    /// client.subject_compatibility_update(&subject_name, Compatibility::FULL).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject_compatibility_update(
        &self,
        subject: &SubjectName,
        compatibility: Compatibility,
    ) -> Result<Compatibility> {
        Ok(self
            .put_config_subject(subject.name(), compatibility)
            .await?
            .into())
    }

    /// Get a list of all registered subjects
    ///
    /// ## Returns
    /// Returns a Result of of all registered subjects from the schema registry
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    /// println!("Subjects: {:?}", client.subjects().await);
    /// # }
    /// ```
    pub async fn subjects(&self) -> Result<Vec<String>> {
        self.get_subjects().await
    }

    /// Get a list of all versions of a subject
    ///
    /// ## Returns
    /// Returns a Result of all version ID's of a subject from the schema registry
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::SubjectName;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    /// println!("Available versions: {:?}", client.subject_versions(&subject_name).await);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject_versions(&self, subject: &SubjectName) -> Result<Vec<i32>> {
        self.get_subjects_subject_versions(subject.name()).await
    }

    /// Get subject for specific version
    ///
    /// ## Returns
    /// Returns a Result of the schema for the given subject and version
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::{SubjectName, SubjectVersion};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    ///
    /// // Get the latest version of the schema
    /// let subject = client.subject(&subject_name, SubjectVersion::Latest).await?;
    /// let raw_schema = subject.schema;
    ///
    /// // Get a specific version of the schema
    /// let subject = client.subject(&subject_name, SubjectVersion::Version(1)).await?;
    /// let raw_schema = subject.schema;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject<V>(&self, subject: &SubjectName, version: V) -> Result<Subject>
    where
        V: Into<SubjectVersion>,
    {
        let subject = subject.name();
        let version = version.into();
        self.get_subjects_subject_versions_id(subject, version.to_string())
            .await
    }

    /// Get the raw schema string for the specified version of subject.
    ///
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    /// - `schema`: [RawSchemaWithType], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SchemaType)
    ///
    /// ## Returns
    /// Returns a Result of the raw schema string for the given subject and version
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::SubjectName;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    /// let raw_schema = client.subject_raw_schema(&subject_name, 1).await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject_raw_schema<V>(&self, subject: &SubjectName, version: V) -> Result<String>
    where
        V: Into<SubjectVersion>,
    {
        self.get_subjects_subject_versions_id_schema(subject.name(), version.into().to_string())
            .await
    }

    /// Get all schemas for a subject
    ///    
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    ///
    /// ## Returns
    /// Returns a Result of all schemas for the given subject
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::SubjectName;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    /// let subjects = client.subject_all_schemas(&subject_name).await?;
    /// # Ok(())
    /// # }
    pub async fn subject_all_schemas(&self, subject: &SubjectName) -> Result<Vec<Subject>> {
        let versions = self.subject_versions(&subject).await?;
        let mut subjects = Vec::new();
        for version in versions {
            let subject = self.subject(&subject, version).await?;
            subjects.push(subject);
        }
        Ok(subjects)
    }

    /// Get all schemas for a topic
    ///
    /// ## Arguments
    /// - `topic`: &str/String of the topic name
    ///
    /// ## Returns
    ///
    // pub async fn topic_all_schemas<S>(&self, topic: S) -> Result<(Vec<Subject>,Vec<Subject>)>
    // where
    //     S: AsRef<str>,
    // {
    //     let key_schemas = self.subject_all_schemas((topic.as_ref(), true)).await?;
    //     let value_schemas = self.subject_all_schemas((topic.as_ref(), false)).await?;
    //     Ok(subjects)
    // }

    /// Post a new schema for a (new) subject
    ///
    /// ## Errors
    /// - If the given schema cannot be converted into a String with given schema type
    /// - The API call will retun a error when
    ///     - subject already has a schema and it's compatibility does not allow it
    ///     - subject already has a schema with a different schema type
    ///     - schema is invalid
    ///
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    /// - `schema`: [RawSchemaWithType], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SchemaType)
    ///
    /// ## Returns
    /// Returns a Result of the new schema ID.
    /// If schema already exists, it will return with the existing schema ID.
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::{RawSchemaWithType, SubjectName};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    ///
    /// // Get subjectname (note it ends on "-value")
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    ///
    /// // You can provide the schema as a raw string (Schema type is optional, it will be detected automatically)
    /// let raw_schema = r#"{ "type": "record", "name": "User", "fields": [ { "name": "name", "type": "string" } ] }"#;
    /// let schema_with_type:RawSchemaWithType = raw_schema.try_into()?;
    /// let schema_version = client.subject_add_schema(&subject_name, schema_with_type).await?;
    ///
    /// // Or if you have a schema object
    /// let avro_schema:RawSchemaWithType  = apache_avro::Schema::parse_str(raw_schema)?.try_into()?; // or ProtoBuf or JSON schema
    /// let schema_version = client.subject_add_schema(&subject_name, avro_schema).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject_add_schema(
        &self,
        subject: &SubjectName,
        schema: RawSchemaWithType,
    ) -> Result<i32> {
        Ok(self
            .post_subjects_subject_versions(subject.name(), schema)
            .await?
            .id())
    }

    /// Check if schema already been registred for a subject
    ///
    /// If it returns 404, it means the schema is not yet registered (even when it states "unable to process")
    ///
    /// ## Errors
    /// - If the given schema cannot be converted into a String with given schema type
    /// - The API call will retun a error when
    ///     - provided schema is different
    ///     - schema is invalid
    ///
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    /// - `schema`: [RawSchemaWithType], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SchemaType)
    ///
    /// ## Returns
    /// If schema exists, it will return with the existing version and schema ID.
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::{SubjectName, SchemaType, RawSchemaWithType};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    ///
    /// // You can provide the schema as a raw string (Schema type is optional, it will be detected automatically)
    /// let raw_schema: RawSchemaWithType = r#"{ "type": "record", "name": "User", "fields": [ { "name": "name", "type": "string" } ] }"#.try_into()?;
    /// let subject_name: SubjectName = "scratch.example-topic.tenant-value".try_into()?;
    /// let subject = client.subject_schema_exist(&subject_name, raw_schema).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subject_schema_exist(
        &self,
        subject: &SubjectName,
        schema: RawSchemaWithType,
    ) -> Result<Subject> {
        self.post_subjects_subject(subject.name(), schema).await
    }

    /// Check if schema is compatible with a specific version of a subject based on the compatibility level
    ///
    /// Note that the compatibility level applied for the check is the configured compatibility level for the subject.
    /// If this subject’s compatibility level was never changed, then the global compatibility level applies.
    ///
    /// ## Arguments
    /// - `subject`: [SubjectName], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SubjectStrategy)
    /// - `version`: Anything that can be converted into a [SubjectVersion]
    /// - `schema`: [RawSchemaWithType], use [TryInto] to convert from &str/String (Returns [SchemaStoreError] error if invalid SchemaType)
    ///
    /// ## Returns
    /// Returns a Result of a boolean if the schema is compatible with the given version of the subject
    pub async fn subject_new_schema_compatibility<Sv>(
        &self,
        subject: &SubjectName,
        version: Sv,
        schema: RawSchemaWithType,
    ) -> Result<bool>
    where
        Sv: Into<SubjectVersion>,
    {
        Ok(self
            .post_compatibility_subjects_subject_versions_id(
                subject.name(),
                version.into().to_string(),
                schema,
            )
            .await?
            .is_compatible())
    }

    /// Get the schema based in schema ID.
    ///
    /// ## Arguments
    /// - `id`: The schema ID (Into<[i32]>)
    pub async fn schema<Si>(&self, id: Si) -> Result<RawSchemaWithType>
    where
        Si: Into<i32>,
    {
        self.get_schemas_ids_id(id.into()).await
    }

    /// Get all subjects that are using the given schema
    ///
    /// ## Arguments
    /// - `id`: The schema ID (Into<[i32]>)
    pub async fn schema_subjects<Si>(&self, id: Si) -> Result<Vec<SubjectVersionInfo>>
    where
        Si: Into<i32>,
    {
        self.get_schemas_ids_id_versions(id.into()).await
    }
}
