use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetRecordQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    projection: Option<Vec<String>>,

    #[serde(rename = "cellFormat", skip_serializing_if = "Option::is_none")]
    cell_format: Option<CellFormats>,

    #[serde(rename = "fieldKeyType", skip_serializing_if = "Option::is_none")]
    field_key: Option<FieldKeyType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CellFormats {
    Json,
    Text,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldKeyType {
    Id,
    Name,
    #[serde(rename = "dbFieldName")]
    DBFieldName,
}
