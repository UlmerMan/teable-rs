use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
}
