use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    #[serde(rename = "singleLineText")]
    SingleLineText,
    #[serde(rename = "longText")]
    LongText,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "attachment")]
    Attachment,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "multipleSelect")]
    MultipleSelect,
    #[serde(rename = "singleSelect")]
    SingleSelect,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "formula")]
    Formula,
    #[serde(rename = "rollup")]
    Rollup,
    #[serde(rename = "conditionalRollup")]
    ConditionalRollup,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "createdTime")]
    CreatedTime,
    #[serde(rename = "lastModifiedTime")]
    LastModifiedTime,
    #[serde(rename = "createdBy")]
    CreatedBy,
    #[serde(rename = "lastModifiedBy")]
    LastModifiedBy,
    #[serde(rename = "autoNumber")]
    AutoNumber,
    #[serde(rename = "button")]
    Button,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CellValueType {
    String,
    Number,
    Boolean,
    DateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DBFieldType {
    TEXT,
    INTEGER,
    DATETIME,
    REAL,
    BLOB,
    JSON,
    BOOLEAN,
}
