use serde::{Deserialize, Serialize};

use crate::models::base::Base;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "avatar")]
    pub avatar: String,

    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "type")]
    pub user_type: String,
    #[serde(rename = "ressourceType")]
    pub resource_type: String,
    #[serde(rename = "isSystem")]
    pub is_system: bool,
    #[serde(rename = "billable")]
    pub billable: bool,
    #[serde(rename = "base")]
    pub base: Base,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetCollaboratorsResponse {
    pub collaborators: Vec<Collaborator>,
    pub total: u32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GetCollaboratorsQuery {
    #[serde(rename = "includeSystem", skip_serializing_if = "Option::is_none")]
    pub include_system: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub take: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Vec<String>>,
}
