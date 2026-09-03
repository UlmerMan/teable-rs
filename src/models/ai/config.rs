use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    #[serde(rename = "modelKey")]
    pub model_key: String,

    #[serde(rename = "type")]
    pub ai_config_type: AiConfigType,

    #[serde(rename = "sourceFieldId")]
    pub source_field_id: String,

    #[serde(rename = "isAutoFill")]
    pub is_auto_fill: Option<bool>,

    #[serde(rename = "attachPrompt")]
    pub attach_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiConfigType {
    Extraxtion,
}