use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use serde::{Serialize, Deserialize};

type Orders = HashMap<String, Order>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub id: String,
    pub label_code: String,
    pub origin: String,
    pub destiny: String,
}

#[derive(Clone)]
pub struct Store {
    pub grocery_list: Arc<RwLock<Orders>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}