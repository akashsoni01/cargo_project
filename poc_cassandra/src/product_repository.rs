use crate::cassandra_manager::CassandraManager;
use crate::product::Product;
use uuid::Uuid;
use std::error::Error;
use log::{info, warn, error};
use std::sync::Arc;

/// ProductRepository handles all Product CRUD operations
/// Uses a dedicated keyspace: "product_keyspace"
pub struct ProductRepository {
    manager: Arc<CassandraManager>,
}

impl ProductRepository {
    /// Create a new ProductRepository instance
    pub fn new(connection_string: String) -> Arc<Self> {
        info!("Initializing ProductRepository with keyspace: product_keyspace");
        let manager = CassandraManager::new(connection_string, Some("product_keyspace".to_string()));
        Arc::new(Self { manager })
    }

    /// Get the underlying CassandraManager
    pub fn manager(&self) -> &Arc<CassandraManager> {
        &self.manager
    }

    /// Initialize the repository (create keyspace and table if needed)
    pub async fn initialize(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Initializing ProductRepository database schema...");
        
        // Wait for connection
        let mut attempts = 0;
        while !self.manager.is_connected().await && attempts < 30 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            attempts += 1;
        }

        if !self.manager.is_connected().await {
            return Err("Failed to connect to Cassandra for ProductRepository".into());
        }

        // Create table
        match self.manager.create_table(
            "products",
            "id UUID PRIMARY KEY, name TEXT, description TEXT, price DOUBLE, stock INT"
        ).await {
            Ok(_) => {
                info!("✓ Products table created successfully");
                Ok(())
            }
            Err(e) => {
                warn!("Failed to create products table: {}", e);
                Err(e)
            }
        }
    }

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

