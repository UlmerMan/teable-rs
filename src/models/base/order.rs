use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    #[serde(rename = "anchorId")]
    pub anchor_id: String,

    #[serde(rename = "position")]
    pub position: Position,
}

#[derive(Debug, Clone, Serialize)]
pub enum Position {
    #[serde(rename = "before")]
    Before,

    #[serde(rename = "after")]
    After,
}
