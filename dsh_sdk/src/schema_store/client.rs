use super::api::SchemaStoreApi;
use super::request::Request;
use super::types::*;
use super::SchemaStoreError;
use crate::Dsh;

/// A high-level client for interacting with the DSH Schema Store API.
///
/// This client wraps various schema registry operations, such as:
/// - Retrieving or setting a subject’s compatibility level.
/// - Listing all subjects and versions.
/// - Fetching a specific schema (by subject/version or by schema ID).
/// - Adding new schemas or checking if they’re already registered.
/// - Verifying schema compatibility against an existing subject/version.
///
/// By default, the client’s base URL is derived from [`Dsh::get().schema_registry_host()`].
/// You can override this behavior with [`SchemaStoreClient::new_with_base_url`].
///
/// Most methods return a [`Result<T, SchemaStoreError>`], which encapsulates
/// potential network failures or schema parsing issues.
///
/// # Example
/// ```no_run
/// use dsh_sdk::schema_store::{SchemaStoreClient, types::SubjectName};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SchemaStoreClient::new();
///     let subject_name: SubjectName = "my.topic-name.tenant-value".try_into()?;
///     let subjects = client.subjects().await?;
///     println!("All subjects: {:?}", subjects);
///     let versions = client.subject_versions(&subject_name).await?;
///     println!("Versions of {:?}: {:?}", subject_name, versions);
///     Ok(())
/// }
/// ```
pub struct SchemaStoreClient<C: Request> {
    pub(crate) base_url: String,
    pub(crate) client: C,
}

impl SchemaStoreClient<reqwest::Client> {
    /// Creates a new `SchemaStoreClient` using the default schema registry URL from
    /// [`Dsh::get().schema_registry_host()`].
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SchemaStoreClient::new();
    ///     // Use `client` to interact with the schema store...
    /// }
    /// ```
    pub fn new() -> Self {
        Self::new_with_base_url(Dsh::get().schema_registry_host())
    }

    /// Creates a `SchemaStoreClient` with a **custom** base URL.
    ///
    /// This is useful if you want to target a non-default or test endpoint.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let base_url = "http://my.custom-registry/api";
    ///     let client = SchemaStoreClient::new_with_base_url(base_url);
    /// }
    /// ```
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
    /// Retrieves the compatibility level for a given subject.
    ///
    /// # Returns
    /// - `Ok(Compatibility)` if successful, representing the subject’s configured compatibility level.
    ///
    /// # Arguments
    /// - `subject`: A [`SubjectName`]. Conversion from a `&str` or `String` can be done via `try_into()`.
    ///
    /// # Errors
    /// Returns [`SchemaStoreError`] if the request fails or if the subject name is invalid.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::SubjectName};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject_name: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     let comp = client.subject_compatibility(&subject_name).await?;
    ///     println!("Subject compatibility: {:?}", comp);
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_compatibility(
        &self,
        subject: &SubjectName,
    ) -> Result<Compatibility, SchemaStoreError> {
        Ok(self.get_config_subject(subject.name()).await?.into())
    }

    /// Sets (updates) the compatibility level for a given subject.
    ///
    /// - If the subject has no existing schema, you can set any compatibility.
    /// - If the subject already has schemas, you can only switch to a **less restrictive** level.
    ///
    /// # Returns
    /// - `Ok(Compatibility)` representing the **new** compatibility level.
    ///
    /// # Errors
    /// Returns [`SchemaStoreError`] if the network call fails, if the subject doesn’t exist,
    /// or if the requested compatibility is not allowed.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::{SubjectName, Compatibility}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     client.subject_compatibility_update(&subject, Compatibility::FULL).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_compatibility_update(
        &self,
        subject: &SubjectName,
        compatibility: Compatibility,
    ) -> Result<Compatibility, SchemaStoreError> {
        Ok(self
            .put_config_subject(subject.name(), compatibility)
            .await?
            .into())
    }

    /// Lists **all** registered subjects in the schema registry.
    ///
    /// # Returns
    /// - `Ok(Vec<String>)` containing subject names.
    ///
    /// # Errors
    /// Returns [`SchemaStoreError`] if the HTTP request or JSON deserialization fails.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = SchemaStoreClient::new();
    ///     match client.subjects().await {
    ///         Ok(subjs) => println!("Registered subjects: {:?}", subjs),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn subjects(&self) -> Result<Vec<String>, SchemaStoreError> {
        self.get_subjects().await
    }

    /// Retrieves the version IDs for a specified subject.
    ///
    /// # Returns
    /// - `Ok(Vec<i32>)` containing the version numbers registered for this subject.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::SubjectName};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     let versions = client.subject_versions(&subject).await?;
    ///     println!("Subject versions: {:?}", versions);
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_versions(
        &self,
        subject: &SubjectName,
    ) -> Result<Vec<i32>, SchemaStoreError> {
        self.get_subjects_subject_versions(subject.name()).await
    }

    /// Fetches a specific schema for a given subject at a specified version.
    ///
    /// - Use [`SubjectVersion::Latest`] for the latest version.
    /// - Use [`SubjectVersion::Version(i32)`] for a specific numbered version.
    ///
    /// # Returns
    /// - `Ok(Subject)` containing metadata and the schema content.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::{SubjectName, SubjectVersion}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     // Latest version
    ///     let latest = client.subject(&subject, SubjectVersion::Latest).await?;
    ///     // Specific version
    ///     let specific = client.subject(&subject, SubjectVersion::Version(2)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject<V>(
        &self,
        subject: &SubjectName,
        version: V,
    ) -> Result<Subject, SchemaStoreError>
    where
        V: Into<SubjectVersion>,
    {
        let subject = subject.name();
        let version = version.into();
        self.get_subjects_subject_versions_id(subject, version.to_string())
            .await
    }

    /// Retrieves **only** the raw schema string for a specified subject version.
    ///
    /// This is useful if you only need the JSON/Avro/Protobuf text, without additional metadata.
    ///
    /// # Returns
    /// - `Ok(String)` containing the schema definition in its raw form.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::SubjectName};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     let raw = client.subject_raw_schema(&subject, 1).await?;
    ///     println!("Schema text: {}", raw);
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_raw_schema<V>(
        &self,
        subject: &SubjectName,
        version: V,
    ) -> Result<String, SchemaStoreError>
    where
        V: Into<SubjectVersion>,
    {
        self.get_subjects_subject_versions_id_schema(subject.name(), version.into().to_string())
            .await
    }

    /// Retrieves **all** schema versions for a specified subject, returning a vector of [`Subject`].
    ///
    /// This method simply calls [`subject_versions`](Self::subject_versions) and then iterates
    /// over each version to fetch the schema details.  
    /// _Note: This can be more expensive than retrieving a single version._
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::SubjectName};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     let all_schemas = client.subject_all_schemas(&subject).await?;
    ///     println!("All schemas: {:?}", all_schemas);
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_all_schemas(
        &self,
        subject: &SubjectName,
    ) -> Result<Vec<Subject>, SchemaStoreError> {
        let versions = self.subject_versions(subject).await?;
        let mut subjects = Vec::new();
        for version in versions {
            let subject_schema = self.subject(subject, version).await?;
            subjects.push(subject_schema);
        }
        Ok(subjects)
    }

    /// Registers a **new** schema under the given subject.
    ///
    /// - If the subject doesn’t exist, it is created with the provided schema.
    /// - If the subject **does** exist and is incompatible with this schema, the registry
    ///   returns an error. If the schema is identical, the existing ID is returned.
    ///
    /// # Returns
    /// - `Ok(i32)` containing the new or existing schema ID.
    ///
    /// # Errors
    /// - If the schema can’t be converted into a valid `RawSchemaWithType`.
    /// - If the API call fails due to network/compatibility issues.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::{RawSchemaWithType, SubjectName}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     let raw_schema = r#"{"type":"record","name":"User","fields":[{"name":"name","type":"string"}]}"#;
    ///     let schema_with_type: RawSchemaWithType = raw_schema.try_into()?;
    ///     let schema_id = client.subject_add_schema(&subject, schema_with_type).await?;
    ///     println!("Schema ID: {}", schema_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_add_schema(
        &self,
        subject: &SubjectName,
        schema: RawSchemaWithType,
    ) -> Result<i32, SchemaStoreError> {
        Ok(self
            .post_subjects_subject_versions(subject.name(), schema)
            .await?
            .id())
    }

    /// Checks if a given schema already exists under the specified subject.
    ///
    /// - Returns 404 if the schema is not registered under that subject.
    /// - Returns [`Subject`] info (including schema ID) if it **is** already present.
    ///
    /// # Returns
    /// - `Ok(Subject)` if the schema matches an existing registration.
    /// - `Err(SchemaStoreError)` if the request fails or the schema is invalid.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::{SubjectName, RawSchemaWithType}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let raw_schema = r#"{"type":"record","name":"User","fields":[{"name":"age","type":"int"}]}"#;
    ///     let schema: RawSchemaWithType = raw_schema.try_into()?;
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     match client.subject_schema_exist(&subject, schema).await {
    ///         Ok(existing) => println!("Schema already registered: {:?}", existing.id),
    ///         Err(e) => eprintln!("Not found or error: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_schema_exist(
        &self,
        subject: &SubjectName,
        schema: RawSchemaWithType,
    ) -> Result<Subject, SchemaStoreError> {
        self.post_subjects_subject(subject.name(), schema).await
    }

    /// Checks if a **new** schema is compatible with a specific version of the subject.
    ///
    /// This leverages the configured compatibility level for the subject (or global level if none is explicitly set).
    ///
    /// # Returns
    /// - `Ok(bool)` indicating whether the new schema is compatible (`true`) or incompatible (`false`).
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::{SchemaStoreClient, types::{SubjectName, RawSchemaWithType, SubjectVersion}};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = SchemaStoreClient::new();
    ///     let raw_schema = r#"{"type":"record","name":"User","fields":[{"name":"name","type":"string"}]}"#;
    ///     let schema: RawSchemaWithType = raw_schema.try_into()?;
    ///     let subject: SubjectName = "example-topic.tenant-value".try_into()?;
    ///     let is_compatible = client.subject_new_schema_compatibility(&subject, SubjectVersion::Latest, schema).await?;
    ///     println!("Is compatible? {}", is_compatible);
    ///     Ok(())
    /// }
    /// ```
    pub async fn subject_new_schema_compatibility<Sv>(
        &self,
        subject: &SubjectName,
        version: Sv,
        schema: RawSchemaWithType,
    ) -> Result<bool, SchemaStoreError>
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

    /// Retrieves a schema by its **global** schema ID.
    ///
    /// # Arguments
    /// - `id`: schema ID (`i32`) referencing the global schema registry ID.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let schema = client.schema(123).await?;
    /// println!("Schema content: {}", schema.content());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn schema<Si>(&self, id: Si) -> Result<RawSchemaWithType, SchemaStoreError>
    where
        Si: Into<i32>,
    {
        self.get_schemas_ids_id(id.into()).await
    }

    /// Lists all subjects that use the specified **global** schema ID.
    ///
    /// # Returns
    /// - `Ok(Vec<SubjectVersionInfo>)` detailing each subject and version that references the schema.
    ///
    /// # Example
    /// ```no_run
    /// use dsh_sdk::schema_store::SchemaStoreClient;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SchemaStoreClient::new();
    /// let references = client.schema_subjects(123).await?;
    /// println!("Subjects referencing schema #123: {:?}", references);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn schema_subjects<Si>(
        &self,
        id: Si,
    ) -> Result<Vec<SubjectVersionInfo>, SchemaStoreError>
    where
        Si: Into<i32>,
    {
        self.get_schemas_ids_id_versions(id.into()).await
    }
}
