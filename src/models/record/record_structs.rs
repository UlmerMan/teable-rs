use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::record::FieldKeyType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub id: String,
    pub fields: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "autoNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_number: Option<i32>,

    #[serde(rename = "createdTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,

    #[serde(rename = "lastModifiedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,

    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub undeletable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteRecordRequest {
    pub id: String,
}

impl From<String> for DeleteRecordRequest {
    fn from(id: String) -> Self {
        DeleteRecordRequest { id }
    }
}

impl From<Record> for DeleteRecordRequest {
    fn from(record: Record) -> Self {
        DeleteRecordRequest { id: record.id }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateRecordRequest {
    #[serde(rename = "fieldKeyType")]
    pub field_key_type: FieldKeyType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub typecast: Option<bool>,

    pub record: RecordUpdate,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<RecordOrder>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateRecordsRequest {
    #[serde(rename = "fieldKeyType")]
    pub field_key_type: FieldKeyType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub typecast: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<RecordOrder>,

    pub records: Vec<RecordUpdate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateRecordsResponse {
    pub records: Vec<Record>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordUpdate {
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordOrder {
    #[serde(rename = "viewId")]
    pub view_id: String,
    #[serde(rename = "anchorId")]
    pub anchor_id: String,
    pub position: RecordOrderPosition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RecordOrderPosition {
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
}
