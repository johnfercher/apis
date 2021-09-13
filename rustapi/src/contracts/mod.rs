use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rand::Rng;
use crate::domain::entities::Order;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateOrderRequest {
    pub origin: String,
    pub destiny: String,
}

impl CreateOrderRequest {
    pub fn to_order(self) -> Order {
        let label_code: u16 = rand::thread_rng().gen();

        Order {
            id: Uuid::new_v4().to_string(),
            label_code: label_code.to_string(),
            origin: self.origin,
            destiny: self.destiny,
        }
    }
}