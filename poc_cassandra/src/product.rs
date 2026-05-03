use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}

impl Product {
    /// Create a new Product instance
    pub fn new(name: String, description: String, price: f64, stock: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            price,
            stock,
        }
    }

    /// Create a Product with a specific ID
    pub fn with_id(id: Uuid, name: String, description: String, price: f64, stock: i32) -> Self {
        Self {
            id,
            name,
            description,
            price,
            stock,
        }
    }
}

