use std::collections::HashMap;
use teable::client::TeableClient;
use teable::models::base::{PostBaseRequest, UpdateBaseRequest};
use teable::models::field::Field;
use teable::models::field::field_types::FieldType;
use teable::models::field::options::{FieldOptions, TextOptions};
use teable::models::record::{
    CreateRecordsRequest, FieldKeyType, RecordUpdate, UpdateRecordRequest,
};
use teable::models::table::PostTableRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TEABLE_TOKEN").expect("TEABLE_TOKEN environment variable not set");
    let space_id =
        std::env::var("TEABLE_SPACE_ID").expect("TEABLE_SPACE_ID environment variable not set");

    let client = TeableClient::builder().token(token).build()?;

    println!("Creating base...");

    let request = PostBaseRequest {
        space_id,
        name: "Rust SDK Test Base".to_string(),
        description: Some("Created by teable-rs example".to_string()),
        icon: Some("🦀".to_string()),
        template: None,
    };

    let base = client.bases().post(&request).await?;

    println!("Created base:");
    println!("  id:   {}", base.id);
    println!("  name: {}", base.name);

    println!();
    println!("Fetching base...");

    let fetched = client.bases().get(&base.id).await?;

    println!("Fetched base:");
    println!("  id:   {}", fetched.id);
    println!("  name: {}", fetched.name);

    assert_eq!(fetched.id, base.id);
    assert_eq!(fetched.name, base.name);

    println!();
    println!("Updating base...");

    let updated = client
        .bases()
        .patch(
            &base.id,
            &UpdateBaseRequest {
                name: Some("Rust SDK Updated Base".to_string()),
                icon: Some("🦀".to_string()),
            },
        )
        .await?;

    println!("Updated base:");
    println!("  name: {}", updated.name);

    println!();
    println!("Creating table...");

    let table = client
        .tables()
        .create(
            &base.id,
            &PostTableRequest {
                name: "Rust SDK Test Table".to_string(),
            },
        )
        .await?;

    println!("Created table:");
    println!("  id:   {}", table.id);
    println!("  name: {}", table.name);

    println!();
    println!("Creating fields...");

    let name_field = client
        .fields()
        .create_field(
            &table.id,
            &Field::builder("", "Name", FieldType::SingleLineText)
                .options(FieldOptions::Text(TextOptions {
                    show_as: None,
                    default_value: None,
                }))
                .build(),
        )
        .await?;

    let notes_field = client
        .fields()
        .create_field(
            &table.id,
            &Field::builder("", "Notes", FieldType::LongText).build(),
        )
        .await?;

    println!("Created fields:");
    println!("  {} ({:?})", name_field.name, name_field.field_type);
    println!("  {} ({:?})", notes_field.name, notes_field.field_type);

    println!();
    println!("Creating record...");

    let mut record_fields = HashMap::new();
    record_fields.insert(
        name_field.id.clone(),
        serde_json::json!("Created from the Rust SDK"),
    );

    let created_response = client
        .records()
        .create_records(
            &table.id,
            &CreateRecordsRequest {
                field_key_type: FieldKeyType::Id,
                typecast: Some(true),
                order: None,
                records: vec![RecordUpdate {
                    fields: record_fields,
                }],
            },
        )
        .await?;

    let created_record = created_response
        .records
        .first()
        .ok_or("create record returned no records")?;

    println!("Created record:");
    println!("  id: {}", created_record.id);

    println!();
    println!("Updating record...");

    let mut updated_fields = HashMap::new();
    updated_fields.insert(
        name_field.id.clone(),
        serde_json::json!("Updated from the Rust SDK"),
    );

    let updated_record = client
        .records()
        .update_record(
            &table.id,
            &created_record.id,
            &UpdateRecordRequest {
                field_key_type: FieldKeyType::Id,
                typecast: Some(true),
                record: RecordUpdate {
                    fields: updated_fields,
                },
                order: None,
            },
        )
        .await?;

    println!("Updated record:");
    println!("  id: {}", updated_record.id);

    println!();
    println!("Listing tables and fields...");

    let tables = client.tables().list_tables(&base.id).await?;
    for table in &tables {
        println!("Table:");
        println!("  id:   {}", table.id);
        println!("  name: {}", table.name);

        let fields = client.fields().list_fields(&table.id).await?;
        println!("  fields: {}", fields.len());
        for field in fields {
            println!("    - {} ({:?})", field.name, field.field_type);
        }
    }

    if let Ok(anchor_id) = std::env::var("TEABLE_ANCHOR_BASE_ID") {
        println!();
        println!("Updating base order...");

        client
            .bases()
            .update_order(
                &base.id,
                &teable::models::base::order::Order {
                    anchor_id,
                    position: teable::models::base::order::Position::After,
                },
            )
            .await?;

        println!("Base order updated successfully.");
    }

    Ok(())
}
