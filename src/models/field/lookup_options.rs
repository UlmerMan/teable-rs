use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LookupOptions {
    Link(LinkLookupOptions),
    Conditional(ConditionalLookupOptions),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkLookupOptions {
    #[serde(rename = "foreignTableId")]
    pub foreign_table_id: String,

    #[serde(rename = "lookupFieldId")]
    pub lookup_field_id: String,

    #[serde(rename = "linkFieldId")]
    pub link_field_id: String,

    pub filter: Option<Filter>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConditionalLookupOptions {
    #[serde(rename = "foreignTableId")]
    pub foreign_table_id: String,

    #[serde(rename = "lookupFieldId")]
    pub lookup_field_id: String,

    pub filter: Filter,

    #[serde(rename = "baseId")]
    pub base_id: Option<String>,

    pub sort: Option<Sort>,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Filter(pub serde_json::Value);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sort(pub serde_json::Value);
