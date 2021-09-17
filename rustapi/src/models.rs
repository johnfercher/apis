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