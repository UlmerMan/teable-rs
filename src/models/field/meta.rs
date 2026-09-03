use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Meta {
    Formula(FormulaMeta),
    Link(LinkMeta),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FormulaMeta {
    #[serde(rename = "persistedAsGeneratedColumn", default)]
    pub persisted_as_generated_column: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkMeta {
    #[serde(rename = "hasOrderColumn", default)]
    pub has_order_column: bool,
}
