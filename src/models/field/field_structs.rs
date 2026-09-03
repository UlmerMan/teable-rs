use serde::{Deserialize, Serialize};

use crate::models::{ai::config::AiConfig, field::field_types::{CellValueType, DBFieldType, FieldType}};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Field {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub options: serde_json::Value,
    
    #[serde(rename = "cellValueType")]
    pub cell_value_type: CellValueType,
    #[serde(rename = "dbFieldType")]
    pub db_field_type: DBFieldType,
    #[serde(rename = "dbFieldName")]
    pub db_field_name: String,
    
    pub description: Option<String>,

    pub meta: Option<serde_json::Value>,

    #[serde(rename = "aiConfig")]
    pub ai_config: Option<AiConfig>,

    #[serde(rename = "isLookup")]
    pub is_lookup: Option<bool>,

    #[serde(rename = "isConditionalLookup")]
    pub is_conditional_lookup: Option<bool>,

    #[serde(rename = "lookupOptions")]
    pub lookup_options: Option<serde_json::Value>,

    #[serde(rename = "notNull")]
    pub not_null: Option<bool>,

    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,

    #[serde(rename = "isComputed")]
    pub is_computed: Option<bool>,

    #[serde(rename = "isPending")]
    pub is_pending: Option<bool>,

    #[serde(rename = "hasError")]
    pub has_error: Option<bool>,

    #[serde(rename = "isMultipleCellValue")]
    pub is_multiple_cell_value: Option<bool>,

    #[serde(rename = "recordRead")]
    pub record_read: Option<bool>,

    #[serde(rename = "recordCreate")]
    pub record_create: Option<bool>,
}