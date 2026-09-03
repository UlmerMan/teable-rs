use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    pub id: String,
    pub name: String,

    #[serde(rename = "dbTableName")]
    pub db_table_name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub order: Option<i32>,
    
    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "defaultViewId")]
    pub default_view_id: Option<String>,
}