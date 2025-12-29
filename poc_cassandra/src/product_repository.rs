use crate::cassandra_manager::CassandraManager;
use crate::repository::Repository;
use crate::product::Product;
use uuid::Uuid;
use std::error::Error;
use log::{info, warn, error};
use std::sync::Arc;

/// ProductRepository handles all Product CRUD operations
/// Uses a dedicated keyspace: "product_keyspace"
/// Connection issues in this repository do not affect other repositories
pub struct ProductRepository {
    manager: Arc<CassandraManager>,
    keyspace: String,
}

impl ProductRepository {
    /// Create a new ProductRepository instance
    /// Each repository has its own independent CassandraManager instance
    /// This ensures connection problems in one repository don't affect others
    pub fn new(connection_string: String) -> Arc<Self> {
        let keyspace = "product_keyspace".to_string();
        info!("Initializing ProductRepository with keyspace: {}", keyspace);
        let manager = CassandraManager::new(connection_string, Some(keyspace.clone()));
        Arc::new(Self { manager, keyspace })
    }

    /// Get the underlying CassandraManager
    pub fn manager(&self) -> &Arc<CassandraManager> {
        &self.manager
    }
}

impl Repository for ProductRepository {
    /// Initialize the repository (create keyspace and table if needed)
    /// This is isolated - failures here don't affect other repositories
    async fn initialize(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Initializing ProductRepository database schema (keyspace: {})...", self.keyspace);
        
        // Wait for connection with timeout - isolated to this repository
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 30;
        
        while !self.manager.is_connected().await && attempts < MAX_ATTEMPTS {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            attempts += 1;
        }

        if !self.manager.is_connected().await {
            warn!("ProductRepository: Failed to connect to Cassandra after {} attempts", MAX_ATTEMPTS);
            return Err(format!("Failed to connect to Cassandra for ProductRepository (keyspace: {})", self.keyspace).into());
        }

        info!("ProductRepository: Connection established, creating schema...");

        // Create table - errors are isolated to this repository
        match self.manager.create_table(
            "products",
            "id UUID PRIMARY KEY, name TEXT, description TEXT, price DOUBLE, stock INT"
        ).await {
            Ok(_) => {
                info!("✓ ProductRepository: Products table created successfully in keyspace '{}'", self.keyspace);
                Ok(())
            }
            Err(e) => {
                warn!("ProductRepository: Failed to create products table: {}", e);
                Err(e)
            }
        }
    }

    /// Check if the repository is connected to Cassandra
    async fn is_connected(&self) -> bool {
        self.manager.is_connected().await
    }

    /// Get the keyspace name used by this repository
    fn keyspace_name(&self) -> &str {
        &self.keyspace
    }
}

impl ProductRepository {
    /// Create a new product (INSERT)
    pub async fn create(&self, product: &Product) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Creating product: {} (${})", product.name, product.price);
        
        let values = format!("{}, '{}', '{}', {}, {}", 
            product.id, 
            product.name.replace("'", "''"), 
            product.description.replace("'", "''"), 
            product.price, 
            product.stock
        );
        
        match self.manager.insert("products", "id, name, description, price, stock", &values).await {
            Ok(_) => {
                info!("✓ Product '{}' created successfully", product.name);
                Ok(())
            }
            Err(e) => {
                error!("✗ Failed to create product '{}': {}", product.name, e);
                Err(e)
            }
        }
    }

    /// Get a product by ID (SELECT)
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Product>, Box<dyn Error + Send + Sync>> {
        info!("Getting product by ID: {}", id);
        
        // Use the manager's select method
        match self.manager.select("products", Some("id, name, description, price, stock"), Some(&format!("id = {}", id))).await {
            Ok(_result) => {
                info!("Query executed successfully for product ID: {}", id);
                // TODO: Parse rows from result when QueryResult API is confirmed
                Ok(None)
            }
            Err(e) => {
                warn!("Query execution failed: {}", e);
                Err(e)
            }
        }
    }

    /// Get a product by name (SELECT)
    pub async fn get_by_name(&self, name: &str) -> Result<Option<Product>, Box<dyn Error + Send + Sync>> {
        info!("Getting product by name: {}", name);
        
        match self.manager.select("products", Some("id, name, description, price, stock"), Some(&format!("name = '{}'", name.replace("'", "''")))).await {
            Ok(_result) => {
                info!("Query executed successfully for product name: {}", name);
                // TODO: Parse rows from result when QueryResult API is confirmed
                Ok(None)
            }
            Err(e) => {
                warn!("Query execution failed: {}", e);
                Err(e)
            }
        }
    }


    /// Get all products (SELECT)
    pub async fn get_all(&self) -> Result<Vec<Product>, Box<dyn Error + Send + Sync>> {
        info!("Getting all products");
        
        match self.manager.select("products", Some("id, name, description, price, stock"), None).await {
            Ok(_result) => {
                info!("Query executed successfully for all products");
                // TODO: Parse rows from result when QueryResult API is confirmed
                Ok(Vec::new())
            }
            Err(e) => {
                warn!("✗ Failed to get all products: {}", e);
                Err(e)
            }
        }
    }

    /// Update a product (UPDATE)
    pub async fn update(&self, id: Uuid, name: Option<&str>, description: Option<&str>, price: Option<f64>, stock: Option<i32>) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Updating product ID: {}", id);
        
        let mut updates = Vec::new();
        
        if let Some(n) = name {
            updates.push(format!("name = '{}'", n.replace("'", "''")));
        }
        if let Some(d) = description {
            updates.push(format!("description = '{}'", d.replace("'", "''")));
        }
        if let Some(p) = price {
            updates.push(format!("price = {}", p));
        }
        if let Some(s) = stock {
            updates.push(format!("stock = {}", s));
        }
        
        if updates.is_empty() {
            return Err("No fields to update".into());
        }
        
        let set_clause = updates.join(", ");
        
        match self.manager.update("products", &set_clause, &format!("id = {}", id)).await {
            Ok(_) => {
                info!("✓ Product {} updated successfully", id);
                Ok(())
            }
            Err(e) => {
                warn!("✗ Failed to update product {}: {}", id, e);
                Err(e)
            }
        }
    }

    /// Delete a product by ID (DELETE)
    pub async fn delete(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Deleting product ID: {}", id);
        
        match self.manager.delete("products", &format!("id = {}", id)).await {
            Ok(_) => {
                info!("✓ Product {} deleted successfully", id);
                Ok(())
            }
            Err(e) => {
                warn!("✗ Failed to delete product {}: {}", id, e);
                Err(e)
            }
        }
    }
}

