use super::types::*;

use super::request::Request;
use super::Result;

use super::SchemaStoreClient;

/// Low level SchemaStoreApi trait
///
/// This trait follows the definition of the Schema Registry API as stated in the OpenAPI specification.
/// It is recomonded to only use the high level [SchemaStoreClient].
pub trait SchemaStoreApi {
    /// Get glabal compatibility level
    ///
    /// {base_url}/config/{subject}
    async fn get_config_subject(&self, subject: String) -> Result<ConfigGet>;
    /// Set compatibility on subject level. With 1 schema stored in the subject, you can change it to any compatibility level. Else, you can only change into a less restrictive level. Must be one of BACKWARD, BACKWARD_TRANSITIVE, FORWARD, FORWARD_TRANSITIVE, FULL, FULL_TRANSITIVE, NONE
    ///
    /// {base_url}/config/{subject}
    async fn put_config_subject(&self, subject: String, body: Compatibility) -> Result<ConfigPut>;
    /// Get a list of registered subjects
    ///
    /// {base_url}/subjects
    async fn get_subjects(&self) -> Result<Vec<String>>;

    /// Check if a schema has already been registered under the specified subject.
    /// If so, this returns the schema string along with its globally unique identifier,
    /// its version under this subject and the subject name: \"{ \\\"schema\\\": {\\\"name\\\": \\\"username\\\", \\\"type\\\": \\\"string\\\"} }\"
    ///
    /// {base_url}/subjects/{subject}
    async fn post_subjects_subject(
        &self,
        subject: String,
        body: RawSchemaWithType,
    ) -> Result<Subject>;

    /// Get a list of versions registered under the specified subject.
    ///
    /// {base_url}/subjects/{subject}
    async fn get_subjects_subject_versions(&self, subject: String) -> Result<Vec<i32>>;

    /// Get a specific version of the schema registered under this subject.
    ///
    /// subjects/{subject}/versions/{id}
    async fn get_subjects_subject_versions_id(
        &self,
        subject: String,
        id: String,
    ) -> Result<Subject>;

    /// Register a new schema under the specified subject.
    ///  
    /// If successfully registered, this returns the unique identifier of this schema in the registry.
    /// The returned identifier should be used to retrieve this schema from the schemas resource and is different from the schema’s version which is associated with the subject.
    /// If the same schema is registered under a different subject, the same identifier will be returned.
    /// However, the version of the schema may be different under different subjects.
    /// A schema should be compatible with the previously registered schema or schemas (if there are any) as per the configured compatibility level.
    /// The configured compatibility level can be obtained by issuing a GET http:get:: /config/(string: subject).
    /// If that returns null, then GET http:get:: /config.
    ///
    /// {base_url}/subjects/{subject}/versions
    async fn post_subjects_subject_versions(
        &self,
        subject: String,
        body: RawSchemaWithType,
    ) -> Result<SchemaId>;

    /// Test input schema against a particular version of a subject’s schema for compatibility.
    /// Note that the compatibility level applied for the check is the configured compatibility level for the subject (GET /config/(string: subject)).
    /// If this subject’s compatibility level was never changed, then the global compatibility level applies (GET /config).
    ///
    /// {base_url}/compatibility/subjects/{subject}/versions/{id}
    async fn post_compatibility_subjects_subject_versions_id(
        &self,
        subject: String,
        id: String,
        body: RawSchemaWithType,
    ) -> Result<CompatibilityCheck>;

    /// "Get the schema for the specified version of this subject. The unescaped schema only is returned.
    ///
    /// {base_url}/subjects/{subject}/versions/{id}/schema
    async fn get_subjects_subject_versions_id_schema(
        &self,
        subject: String,
        version_id: String,
    ) -> Result<String>;

    /// Get the schema for the specified version of schema.
    ///
    /// {base_url}/schemas/ids/{id}
    async fn get_schemas_ids_id(&self, id: i32) -> Result<RawSchemaWithType>;

    /// Get the related subjects vesrion for the specified schema.
    ///
    /// {base_url}/schemas/ids/{id}/versions
    async fn get_schemas_ids_id_versions(&self, id: i32) -> Result<Vec<SubjectVersionInfo>>;
}

impl<C> SchemaStoreApi for SchemaStoreClient<C>
where
    C: Request,
{
    async fn get_config_subject(&self, subject: String) -> Result<ConfigGet> {
        let url = format!("{}/config/{}", self.base_url, subject);
        Ok(self.client.get_request(url).await?)
    }

    async fn put_config_subject(&self, subject: String, body: Compatibility) -> Result<ConfigPut> {
        let url = format!("{}/config/{}", self.base_url, subject);
        Ok(self.client.put_request(url, body).await?)
    }

    async fn get_subjects(&self) -> Result<Vec<String>> {
        let url = format!("{}/subjects", self.base_url);
        Ok(self.client.get_request(url).await?)
    }

    async fn post_subjects_subject(
        &self,
        subject: String,
        body: RawSchemaWithType,
    ) -> Result<Subject> {
        let url = format!("{}/subjects/{}", self.base_url, subject);
        Ok(self.client.post_request(url, body).await?)
    }

    async fn get_subjects_subject_versions(&self, subject: String) -> Result<Vec<i32>> {
        let url = format!("{}/subjects/{}/versions", self.base_url, subject);
        Ok(self.client.get_request(url).await?)
    }

    async fn get_subjects_subject_versions_id(
        &self,
        subject: String,
        version_id: String,
    ) -> Result<Subject> {
        let url = format!(
            "{}/subjects/{}/versions/{}",
            self.base_url, subject, version_id
        );
        Ok(self.client.get_request(url).await?)
    }

    async fn post_subjects_subject_versions(
        &self,
        subject: String,
        body: RawSchemaWithType,
    ) -> Result<SchemaId> {
        let url = format!("{}/subjects/{}/versions", self.base_url, subject);
        Ok(self.client.post_request(url, body).await?)
    }

    async fn post_compatibility_subjects_subject_versions_id(
        &self,
        subject: String,
        version_id: String,
        body: RawSchemaWithType,
    ) -> Result<CompatibilityCheck> {
        let url = format!(
            "{}/compatibility/subjects/{}/versions/{}",
            self.base_url, subject, version_id
        );
        Ok(self.client.post_request(url, body).await?)
    }

    async fn get_subjects_subject_versions_id_schema(
        &self,
        subject: String,
        version_id: String,
    ) -> Result<String> {
        let url = format!(
            "{}/subjects/{}/versions/{}/schema",
            self.base_url, subject, version_id
        );
        Ok(self.client.get_request_plain(url).await?)
    }

    async fn get_schemas_ids_id(&self, id: i32) -> Result<RawSchemaWithType> {
        let url = format!("{}/schemas/ids/{}", self.base_url, id);
        Ok(self.client.get_request(url).await?)
    }

    async fn get_schemas_ids_id_versions(&self, id: i32) -> Result<Vec<SubjectVersionInfo>> {
        let url = format!("{}/schemas/ids/{}/versions", self.base_url, id);
        Ok(self.client.get_request(url).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_config_subject() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("GET", "/config/test-value")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"compatibilityLevel":"FULL"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .get_config_subject("test-value".to_string())
            .await
            .unwrap();
        assert_eq!(result.compatibility_level, Compatibility::FULL);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_put_config_subject() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("PUT", "/config/test-value")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"compatibility":"FULL"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .put_config_subject("test-value".to_string(), Compatibility::FULL)
            .await
            .unwrap();
        assert_eq!(result.compatibility, Compatibility::FULL);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_subjects() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("GET", "/subjects")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"["test-value", "topic-key"]"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client.get_subjects().await.unwrap();
        assert_eq!(result, vec!["test-value", "topic-key"]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_post_subjects_subject() {
        let mut ss = mockito::Server::new_async().await;
        let avro_schema = RawSchemaWithType {
            schema_type: SchemaType::AVRO,
            schema: r#"{"type":"string"}"#.to_string(),
        };
        ss.mock( "POST", "/subjects/test-value")
            .match_body(mockito::Matcher::Json(serde_json::to_value(&avro_schema).unwrap()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subject":"test-value", "version":1, "id":1, "schema":"{\"type\":\"string\"}"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .post_subjects_subject("test-value".to_string(), avro_schema)
            .await
            .unwrap();
        assert_eq!(result.subject, "test-value");
        assert_eq!(result.version, 1);
        assert_eq!(result.id, 1);
        assert_eq!(result.schema, r#"{"type":"string"}"#);
        assert_eq!(result.schema_type, SchemaType::AVRO);

        let proto_schema = RawSchemaWithType {
            schema_type: SchemaType::PROTOBUF,
            schema: r#"syntax = "proto3";package com.kpn.protobuf;message SimpleMessage {string content = 1;string date_time = 2;}"#.to_string(),
        };
        ss.mock( "POST", "/subjects/protobuf-value")
            .match_body(mockito::Matcher::Json(serde_json::to_value(&proto_schema).unwrap()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subject":"protobuf-value", "version":1, "id":1, "schemaType":"PROTOBUF", "schema":"syntax = \"proto3\";package com.kpn.protobuf;message SimpleMessage {string content = 1;string date_time = 2;}"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .post_subjects_subject("protobuf-value".to_string(), proto_schema)
            .await
            .unwrap();
        assert_eq!(result.subject, "protobuf-value");
        assert_eq!(result.version, 1);
        assert_eq!(result.id, 1);
        assert_eq!(
            result.schema,
            r#"syntax = "proto3";package com.kpn.protobuf;message SimpleMessage {string content = 1;string date_time = 2;}"#
        );
        assert_eq!(result.schema_type, SchemaType::PROTOBUF);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_subjects_subject_versions() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("GET", "/subjects/test-value/versions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[1, 2, 3]"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .get_subjects_subject_versions("test-value".to_string())
            .await
            .unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_subjects_subject_versions_id() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock( "GET", "/subjects/test-value/versions/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"subject":"test-value", "version":1, "id":1, "schema":"{\"type\":\"string\"}"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .get_subjects_subject_versions_id("test-value".to_string(), "1".to_string())
            .await
            .unwrap();
        assert_eq!(result.subject, "test-value");
        assert_eq!(result.version, 1);
        assert_eq!(result.id, 1);
        assert_eq!(result.schema, r#"{"type":"string"}"#);
        assert_eq!(result.schema_type, SchemaType::AVRO);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_post_subjects_subject_versions() {
        let mut ss = mockito::Server::new_async().await;
        let avro_schema = RawSchemaWithType {
            schema_type: SchemaType::AVRO,
            schema: r#"{"type":"string"}"#.to_string(),
        };
        ss.mock("POST", "/subjects/test-value/versions")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&avro_schema).unwrap(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":1}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .post_subjects_subject_versions("test-value".to_string(), avro_schema)
            .await
            .unwrap();
        assert_eq!(result.id, 1);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_post_compatibility_subjects_subject_versions_id() {
        let mut ss = mockito::Server::new_async().await;
        let avro_schema = RawSchemaWithType {
            schema_type: SchemaType::AVRO,
            schema: r#"{"type":"string"}"#.to_string(),
        };
        ss.mock("POST", "/compatibility/subjects/test-value/versions/1")
            .match_body(mockito::Matcher::Json(
                serde_json::to_value(&avro_schema).unwrap(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"is_compatible":true}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .post_compatibility_subjects_subject_versions_id(
                "test-value".to_string(),
                "1".to_string(),
                avro_schema,
            )
            .await
            .unwrap();
        assert_eq!(result.is_compatible(), true);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_subjects_subject_versions_id_schema() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("GET", "/subjects/test-value/versions/1/schema")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"type":"string"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client
            .get_subjects_subject_versions_id_schema("test-value".to_string(), "1".to_string())
            .await
            .unwrap();
        assert_eq!(result, r#"{"type":"string"}"#);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_schemas_ids_id() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("GET", "/schemas/ids/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"schema":"{\"type\":\"string\"}"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client.get_schemas_ids_id(1).await.unwrap();
        assert_eq!(result.schema, r#"{"type":"string"}"#);
        assert_eq!(result.schema_type, SchemaType::AVRO);

        ss.mock("GET", "/schemas/ids/2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"schema":"syntax = \"proto3\";package com.kpn.protobuf;message SimpleMessage {string content = 1;string date_time = 2;}", "schemaType": "PROTOBUF"}"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client.get_schemas_ids_id(2).await.unwrap();
        assert_eq!(
            result.schema,
            r#"syntax = "proto3";package com.kpn.protobuf;message SimpleMessage {string content = 1;string date_time = 2;}"#
        );
        assert_eq!(result.schema_type, SchemaType::PROTOBUF);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_schemas_ids_id_versions() {
        let mut ss = mockito::Server::new_async().await;
        ss.mock("GET", "/schemas/ids/1/versions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"subject":"test-value", "version":1, "id":1, "schema":"{\"type\":\"string\"}"}]"#)
            .create();
        let client = SchemaStoreClient::new_with_base_url(&ss.url());
        let result = client.get_schemas_ids_id_versions(1).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].subject, "test-value");
        assert_eq!(result[0].version, 1);
    }
}
