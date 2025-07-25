//! This example demonstrates how to interact with the Schema Store API using the dsh_sdk crate.
//! It uses the SchemaStoreClient to register a schema, retrieve it by ID, and list all registered subjects and schemas.
//!
//! Example is using the following crates:
//! - [`dsh_sdk`] with features = ["schema-store"] for Schema Store API client
//! - [`tokio`] with features = ["full"] for async runtime
//!
//! This example uses the default http://localhost:8081/apis/ccompat/v7 as default when running on a local machine.
//! This is the default Schema Registry URL of a local APICurio instance.
//! On DSH it automatically picks up the Schema Registry URL of DSH Schema Store.
//! You can also set environment variable SCHEMA_REGISTRY_HOST to overwrite the default endpoint.
//!
//! Run this example with:
//! ```bash
//! SCHEMA_REGISTRY_HOST=http://localhost:8081/apis/ccompat/v7 cargo run --features schema-store --example schema_store_api
//! ```

use dsh_sdk::schema_store::SchemaStoreClient;
use dsh_sdk::schema_store::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new SchemaStoreClient, connects to the Schema Registry based on datastreams.json
    // However, you can overwrite it by setting the environment variable SCHEMA_REGISTRY_HOST or SchemaStoreClient::new_with_base_url("http://localhost:8081")
    let client = SchemaStoreClient::new();

    // Register a new schema (and subject if not exists)
    let schema = r#"
    {
        "type": "record",
        "name": "Test",
        "fields": [
            {"name": "name", "type": "string"},
            {"name": "age", "type": "int"}
        ]
    }
    "#;
    let subject_name =
        SubjectName::new_topic_name_strategy("scratch.topic-name.tenant-name", false); // "scratch.topic-name.tenant-name-value"
    let schema: RawSchemaWithType = schema.try_into()?;
    let schema_id = client.subject_add_schema(&subject_name, schema).await?;
    println!("Registered schema with id: {}\n", schema_id);

    // Get schema by id
    let raw_schema = client.schema(schema_id).await?;
    println!("Schema by id {}: {:#?}\n", schema_id, raw_schema);

    // List all subjects
    let schemas = client.subjects().await?;
    println!("List all registred subjects: {:#?}\n", schemas);

    let subject_name: SubjectName = "scratch.topic-name.tenant-name-value".try_into()?;
    // List all schemas for a subject
    let schemas_for_subject = client.subject_all_schemas(&subject_name).await?;
    println!("List all schemas for subject: {:#?}\n", schemas_for_subject);

    // Get the latest schema for a subject
    let latest_schema = client
        .subject_raw_schema(
            &"scratch.topic-name.tenant-name-value".try_into()?,
            SubjectVersion::Latest,
        )
        .await?;
    println!("Latest schema for subject: {:#?}\n", latest_schema);

    Ok(())
}
