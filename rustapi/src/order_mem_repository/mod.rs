use crate::domain::repositories::OrderRepository;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::domain::entities::Order;
use std::io::{Error, ErrorKind};
use async_trait::async_trait;

type Orders = HashMap<String, Order>;

#[derive(Clone)]
pub struct OrderMemRepository {
    pub grocery_list: Arc<RwLock<Orders>>
}

impl OrderMemRepository {
    pub fn new() -> Self {
        OrderMemRepository {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl OrderRepository for OrderMemRepository {
    async fn get_by_id(&self, _: String) -> Result<Order, Error> {
        Err(Error::new(ErrorKind::AddrInUse, "bla"))
    }

    async fn search(&self) -> Result<Vec<Order>, Error> {
        let mut result = Vec::new();
        let r = self.grocery_list.read();

        for (_,value) in r.await.clone().into_iter() {
            result.push(value);
        }

        Ok(result)
    }

    async fn create(&self, order: Order) -> Option<Error> {
        self.grocery_list.write().await.insert(order.id.clone(), order);
        None
    }

    async fn update(&self, order: Order) -> Option<Error> {
        self.grocery_list.write().await.insert(order.id.clone(), order);
        None
    }

    async fn delete(&self, id: String) -> Option<Error> {
        self.grocery_list.write().await.remove(&id);
        None
    }
}