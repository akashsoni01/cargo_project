use crate::cassandra_manager::CassandraManager;
use crate::user::User;
use uuid::Uuid;
use std::error::Error;
use log::{info, warn, error};
use std::sync::Arc;

/// UserRepository handles all User CRUD operations
/// Uses a dedicated keyspace: "user_keyspace"
pub struct UserRepository {
    manager: Arc<CassandraManager>,
}

impl UserRepository {
    /// Create a new UserRepository instance
    pub fn new(connection_string: String) -> Arc<Self> {
        info!("Initializing UserRepository with keyspace: user_keyspace");
        let manager = CassandraManager::new(connection_string, Some("user_keyspace".to_string()));
        Arc::new(Self { manager })
    }

    /// Get the underlying CassandraManager
    pub fn manager(&self) -> &Arc<CassandraManager> {
        &self.manager
    }

    /// Initialize the repository (create keyspace and table if needed)
    pub async fn initialize(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Initializing UserRepository database schema...");
        
        // Wait for connection
        let mut attempts = 0;
        while !self.manager.is_connected().await && attempts < 30 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            attempts += 1;
        }

        if !self.manager.is_connected().await {
            return Err("Failed to connect to Cassandra for UserRepository".into());
        }

        // Create table
        match self.manager.create_table(
            "users",
            "id UUID PRIMARY KEY, name TEXT, email TEXT, password TEXT"
        ).await {
            Ok(_) => {
                info!("✓ Users table created successfully");
                Ok(())
            }
            Err(e) => {
                warn!("Failed to create users table: {}", e);
                Err(e)
            }
        }
    }

    /// Create a new user (INSERT)
    pub async fn create(&self, user: &User) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Creating user: {} ({})", user.name, user.email);
        
        let values = format!("{}, '{}', '{}', '{}'", user.id, user.name, user.email, user.password);
        
        match self.manager.insert("users", "id, name, email, password", &values).await {
            Ok(_) => {
                info!("✓ User '{}' created successfully", user.name);
                Ok(())
            }
            Err(e) => {
                error!("✗ Failed to create user '{}': {}", user.name, e);
                Err(e)
            }
        }
    }

    /// Get a user by ID (SELECT)
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        info!("Getting user by ID: {}", id);
        
        match self.manager.get_user_by_id(id).await {
            Ok(user) => {
                if let Some(u) = &user {
                    info!("✓ User '{}' retrieved successfully", u.name);
                } else {
                    info!("User with ID {} not found", id);
                }
                Ok(user)
            }
            Err(e) => {
                warn!("✗ Failed to get user by ID: {}", e);
                Err(e)
            }
        }
    }

    /// Get a user by email (SELECT)
    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        info!("Getting user by email: {}", email);
        
        match self.manager.get_user_by_email(email).await {
            Ok(user) => {
                if let Some(u) = &user {
                    info!("✓ User '{}' retrieved successfully", u.name);
                } else {
                    info!("User with email {} not found", email);
                }
                Ok(user)
            }
            Err(e) => {
                warn!("✗ Failed to get user by email: {}", e);
                Err(e)
            }
        }
    }

    /// Get all users (SELECT)
    pub async fn get_all(&self) -> Result<Vec<User>, Box<dyn Error + Send + Sync>> {
        info!("Getting all users");
        
        match self.manager.get_all_users().await {
            Ok(users) => {
                info!("✓ Retrieved {} users successfully", users.len());
                Ok(users)
            }
            Err(e) => {
                warn!("✗ Failed to get all users: {}", e);
                Err(e)
            }
        }
    }

    /// Update a user (UPDATE)
    pub async fn update(&self, id: Uuid, name: Option<&str>, email: Option<&str>, password: Option<&str>) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Updating user ID: {}", id);
        
        match self.manager.update_user(id, name, email, password).await {
            Ok(_) => {
                info!("✓ User {} updated successfully", id);
                Ok(())
            }
            Err(e) => {
                warn!("✗ Failed to update user {}: {}", id, e);
                Err(e)
            }
        }
    }

    /// Delete a user by ID (DELETE)
    pub async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Deleting user ID: {}", id);
        
        match self.manager.delete_user(id).await {
            Ok(_) => {
                info!("✓ User {} deleted successfully", id);
                Ok(())
            }
            Err(e) => {
                warn!("✗ Failed to delete user {}: {}", id, e);
                Err(e)
            }
        }
    }

    /// Delete a user by email (DELETE)
    pub async fn delete_by_email(&self, email: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Deleting user with email: {}", email);
        
        match self.manager.delete_user_by_email(email).await {
            Ok(_) => {
                info!("✓ User with email {} deleted successfully", email);
                Ok(())
            }
            Err(e) => {
                warn!("✗ Failed to delete user with email {}: {}", email, e);
                Err(e)
            }
        }
    }
}

