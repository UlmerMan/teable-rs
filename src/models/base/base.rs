use serde::{Deserialize, Serialize};

use super::{
    collaborator_type::CollaboratorType,
    role::Role,
    template::Template,
};

use crate::models::user::User;

/// A Teable base returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Base {
    pub id: String,
    pub name: String,

    pub description: Option<String>,
    pub icon: Option<String>,

    pub role: Option<Role>,

    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,

    #[serde(rename = "collaboratorType")]
    pub collaborator_type: Option<CollaboratorType>,

    #[serde(rename = "restrictedAuthority")]
    pub restricted_authority: Option<bool>,

    #[serde(rename = "enabledAuthority")]
    pub enabled_authority: Option<bool>,

    #[serde(rename = "lastModifiedTime")]
    pub last_modified_time: Option<String>,

    #[serde(rename = "createdTime")]
    pub created_time: Option<String>,

    pub template: Option<Template>,

    #[serde(rename = "createdByUser")]
    pub created_by_user: Option<User>,

    #[serde(rename = "isCanary")]
    pub is_canary: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PostBaseRequest {
    #[serde(rename = "spaceId")]
    pub space_id: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBaseResponse {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    pub name: String,
    pub icon: Option<String>,
}
