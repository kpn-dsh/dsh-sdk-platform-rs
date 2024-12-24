use super::api::SchemaStoreApi;
use super::request::Request;
use super::types::*;
use super::{Result, SchemaStoreError};
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
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    /// println!("Config: {:?}", client.subject_compatibility("scratch.example-topic.tenant-value").await);
    /// # }
    ///
    pub async fn subject_compatibility<Sn>(&self, subject: Sn) -> Result<Compatibility>
    where
        Sn: Into<SubjectName>,
    {
        Ok(self.get_config_subject(subject.into().name()).await?.into())
    }

    /// Set the compatibility level for a subject
    ///
    /// Set compatibility on subject level. With 1 schema stored in the subject, you can change it to any compatibility level.
    /// Else, you can only change into a less restrictive level.
    ///
    /// ## Returns
    /// Returns a Result of the new compatibility level
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::Compatibility;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    /// client.subject_compatibility_update("scratch.example-topic.tenant-value", Compatibility::FULL).await.unwrap();
    /// # }
    /// ```
    ///
    /// TODO: untested as this API method does not seem to work at all on DSH
    pub async fn subject_compatibility_update<Sn>(
        &self,
        subject: Sn,
        compatibility: Compatibility,
    ) -> Result<Compatibility>
    where
        Sn: Into<SubjectName>,
    {
        Ok(self
            .put_config_subject(subject.into().name(), compatibility)
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
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    /// println!("Available versions: {:?}", client.subject_versions("scratch.example-topic.tenant-value").await);
    /// # }
    /// ```
    pub async fn subject_versions<Sn>(&self, subject: Sn) -> Result<Vec<i32>>
    where
        Sn: Into<SubjectName>,
    {
        self.get_subjects_subject_versions(subject.into().name())
            .await
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
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    ///
    /// // Get the latest version of the schema
    /// let subject = client.subject(SubjectName::TopicNameStrategy{topic: "scratch.example-topic.tenant".to_string(), key: false}, SubjectVersion::Latest).await.unwrap();
    /// let raw_schema = subject.schema;
    ///
    /// // Get a specific version of the schema
    /// let subject = client.subject("scratch.example-topic.tenant-value", SubjectVersion::Version(1)).await.unwrap();
    /// let raw_schema = subject.schema;
    /// # }
    /// ```
    pub async fn subject<Sn, V>(&self, subject: Sn, version: V) -> Result<Subject>
    where
        //Sn: TryInto<SubjectName, Error = SchemaStoreError>,
        Sn: Into<SubjectName>,
        V: Into<SubjectVersion>,
    {
        let subject = subject.into().name();
        let version = version.into();
        self.get_subjects_subject_versions_id(subject, version.to_string())
            .await
    }

    /// Get the raw schema string for the specified version of subject.
    ///
    /// ## Arguments
    /// - `subject`: Anything that can be converted into a [SubjectName] (Returns error if invalid SubjectStrategy)
    /// - `schema`: Anything that can be converted into a [RawSchemaWithType]
    ///
    /// ## Returns
    /// Returns a Result of the raw schema string for the given subject and version
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    /// let raw_schema = client.subject_raw_schema("scratch.example-topic.tenant-value", 1).await.unwrap();
    /// # }
    /// ```
    pub async fn subject_raw_schema<Sn, V>(&self, subject: Sn, version: V) -> Result<String>
    where
        Sn: Into<SubjectName>,
        V: Into<SubjectVersion>,
    {
        self.get_subjects_subject_versions_id_schema(
            subject.into().name(),
            version.into().to_string(),
        )
        .await
    }

    /// Get all schemas for a subject
    ///    
    /// ## Arguments
    /// - `subject`: Anything that can be converted into a [SubjectName] (Returns error if invalid SubjectStrategy)
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
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    /// let subjects = client.subject_all_schemas(SubjectName::TopicNameStrategy{topic: "scratch.example-topic.tenant".to_string(), key: false}).await.unwrap();
    /// # }
    pub async fn subject_all_schemas<Sn>(&self, subject: Sn) -> Result<Vec<Subject>>
    where
        Sn: Into<SubjectName> + Clone,
    {
        let versions = self.subject_versions(subject.clone()).await?;
        let mut subjects = Vec::new();
        for version in versions {
            let subject = self.subject(subject.clone(), version).await?;
            subjects.push(subject);
        }
        Ok(subjects)
    }

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
    /// - `subject`: Anything that can be converted into a [SubjectName] (Returns error if invalid SubjectStrategy)
    /// - `schema`: Anything that can be converted into a [RawSchemaWithType]
    ///
    /// ## Returns
    /// Returns a Result of the new schema ID.
    /// If schema already exists, it will return with the existing schema ID.
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::SchemaType};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    ///
    /// // You can provide the schema as a raw string (Schema type is optional, it will be detected automatically)
    /// let raw_schema = r#"{ "type": "record", "name": "User", "fields": [ { "name": "name", "type": "string" } ] }"#;
    /// let schema_version = client.subject_add_schema("scratch.example-topic.tenant-value", (raw_schema, SchemaType::AVRO)).await.unwrap();
    ///
    /// // Or if you have a schema object
    /// let avro_schema = apache_avro::Schema::parse_str(raw_schema).unwrap(); // or ProtoBuf or JSON schema
    /// let schema_version = client.subject_add_schema("scratch.example-topic.tenant-value", avro_schema).await.unwrap();
    /// # }
    /// ```
    pub async fn subject_add_schema<Sn, Sc>(&self, subject: Sn, schema: Sc) -> Result<i32>
    where
        Sn: Into<SubjectName>,
        Sc: TryInto<RawSchemaWithType, Error = SchemaStoreError>,
    {
        let schema = schema.try_into()?;
        Ok(self
            .post_subjects_subject_versions(subject.into().name(), schema)
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
    /// - `subject`: Anything that can be converted into a [SubjectName] (Returns error if invalid SubjectStrategy)
    /// - `schema`: Anything that can be converted into a [RawSchemaWithType]
    ///
    /// ## Returns
    /// If schema exists, it will return with the existing version and schema ID.
    ///
    /// ## Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// use dsh_sdk::schema_store::types::{SubjectName, SchemaType};
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = SchemaStoreClient::new();
    ///
    /// // You can provide the schema as a raw string (Schema type is optional, it will be detected automatically)
    /// let raw_schema = r#"{ "type": "record", "name": "User", "fields": [ { "name": "name", "type": "string" } ] }"#;
    /// let subject = client.subject_schema_exist("scratch.example-topic.tenant-value", (raw_schema, SchemaType::AVRO)).await.unwrap();
    /// # }
    /// ```
    pub async fn subject_schema_exist<Sn, Sc>(&self, subject: Sn, schema: Sc) -> Result<Subject>
    where
        Sn: Into<SubjectName>,
        Sc: TryInto<RawSchemaWithType, Error = SchemaStoreError>,
    {
        let schema = schema.try_into()?;
        self.post_subjects_subject(subject.into().name(), schema)
            .await
    }

    /// Check if schema is compatible with a specific version of a subject based on the compatibility level
    ///
    /// Note that the compatibility level applied for the check is the configured compatibility level for the subject.
    /// If this subject’s compatibility level was never changed, then the global compatibility level applies.
    ///
    /// ## Arguments
    /// - `subject`: Anything that can be converted into a [SubjectName] (Returns error if invalid SubjectStrategy)
    /// - `version`: Anything that can be converted into a [SubjectVersion]
    /// - `schema`: Anything that can be converted into a [RawSchemaWithType]
    ///
    /// ## Returns
    /// Returns a Result of a boolean if the schema is compatible with the given version of the subject
    pub async fn subject_new_schema_compatibility<Sn, Sv, Sc>(
        &self,
        subject: Sn,
        version: Sv,
        schema: Sc,
    ) -> Result<bool>
    where
        Sn: Into<SubjectName>,
        Sv: Into<SubjectVersion>,
        Sc: TryInto<RawSchemaWithType, Error = SchemaStoreError>,
    {
        let schema = schema.try_into()?;
        Ok(self
            .post_compatibility_subjects_subject_versions_id(
                subject.into().name(),
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
