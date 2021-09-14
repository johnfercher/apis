use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::domain::entities::Order;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderRequest {
    pub label_code: String,
    pub origin: String,
    pub destiny: String,
}

impl OrderRequest {
    pub fn to_order(self, id: String) -> Order {
        Order {
            id,
            label_code: self.label_code,
            origin: self.origin,
            destiny: self.destiny,
        }
    }
}