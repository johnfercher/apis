use super::schema::orders;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Clone)]
#[table_name="orders"]
pub struct Order {
    pub id: String,
    pub label_code: String,
    pub origin: String,
    pub destiny: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct IdResponse {
    pub id: String,
}

impl IdResponse {
    pub fn new(id: String) -> IdResponse {
        IdResponse { id }
    }
}