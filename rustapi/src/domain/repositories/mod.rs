use crate::domain::entities::Order;
use std::io::Error;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository {
    async fn get_by_id(&self, id: String) -> Result<Order, Error>;
    async fn search(&self) -> Result<Vec<Order>, Error>;
    async fn create(&self, order: Order) -> Option<Error>;
    async fn update(&self, order: Order) -> Option<Error>;
    async fn delete(&self, id: String) -> Option<Error>;
}
