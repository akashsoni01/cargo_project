use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    /// Create a new User instance
    pub fn new(name: String, email: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            email,
            password,
        }
    }

    /// Create a User with a specific ID
    pub fn with_id(id: Uuid, name: String, email: String, password: String) -> Self {
        Self {
            id,
            name,
            email,
            password,
        }
    }
}

