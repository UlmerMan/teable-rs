use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    pub id: String,
    pub name: String,

    #[serde(rename = "dbTableName")]
    pub db_table_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    
    #[serde(rename = "lastModifiedTime", skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "defaultViewId", skip_serializing_if = "Option::is_none")]
    pub default_view_id: Option<String>,
}