use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub id: String,
    pub label_code: String,
    pub origin: String,
    pub destiny: String,
}