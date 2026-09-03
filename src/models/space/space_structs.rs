use serde::{Deserialize, Serialize};

use crate::models::user::{Organization, Role};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub role: Option<Role>,
    pub organization: Option<Organization>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostSpaceRequest {
    pub name: String,
}

impl From<String> for PostSpaceRequest {
    fn from(name: String) -> Self {
        PostSpaceRequest { name }
    }
}

impl From<Space> for PostSpaceRequest {
    fn from(space: Space) -> Self {
        PostSpaceRequest { name: space.name }
    }
}
