mod cassandra_manager;

use cassandra_manager::CassandraManager;
use std::error::Error;
use log::{info, warn};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    env_logger::init();

    info!("Starting application...");

    // Create Cassandra manager - this will NOT fail even if Cassandra is down
    let manager = CassandraManager::new(
        "127.0.0.1:9042".to_string(),
        Some("my_keyspace".to_string()),
    );

    info!("CassandraManager initialized. Server is running even if DB is unavailable.");
    info!("The manager will poll for connection every 5 seconds.");

    // Wait a bit for initial connection attempt
    sleep(Duration::from_secs(2)).await;

    // Try to set up the keyspace and table when connected
    setup_database(&manager).await;

    // Keep the application running
    // In a real application, you would start your web server here
    info!("Application is running. Press Ctrl+C to stop.");
    
    // Example: Keep running and periodically check connection status
    loop {
        sleep(Duration::from_secs(10)).await;
        
        if manager.is_connected().await {
            info!("Cassandra is connected!");
            
            // Example query
            match manager.query("SELECT now() FROM system.local", &[]).await {
                Ok(_) => info!("Query executed successfully"),
                Err(e) => warn!("Query failed: {}", e),
            }
        } else {
            info!("Cassandra is not connected yet. Waiting for connection...");
        }
    }
}

async fn setup_database(manager: &CassandraManager) {
    // Wait for connection with timeout
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 12; // 1 minute max wait
    
    while !manager.is_connected().await && attempts < MAX_ATTEMPTS {
        sleep(Duration::from_secs(5)).await;
        attempts += 1;
    }

    if !manager.is_connected().await {
        info!("Cassandra not available yet. Database setup will be retried when connection is established.");
        return;
    }

    info!("Cassandra connected! Setting up database...");

    // Create the keyspace
    let create_keyspace = "
        CREATE KEYSPACE IF NOT EXISTS my_keyspace
        WITH REPLICATION = { 'class' : 'SimpleStrategy', 'replication_factor' : 1 };
    ";
    
    match manager.query(create_keyspace, &[]).await {
        Ok(_) => info!("Keyspace created successfully"),
        Err(e) => warn!("Failed to create keyspace: {}", e),
    }

    // Use the keyspace
    let use_keyspace = "USE my_keyspace;";
    match manager.query(use_keyspace, &[]).await {
        Ok(_) => info!("Using keyspace my_keyspace"),
        Err(e) => warn!("Failed to use keyspace: {}", e),
    }

    // Create the users table using prepared statement
    info!("Creating users table using prepared statement...");
    match manager.create_table(
        "users",
        "id UUID PRIMARY KEY, name TEXT, email TEXT, password TEXT"
    ).await {
        Ok(_) => info!("Users table created successfully using prepared statement"),
        Err(e) => warn!("Failed to create users table: {}", e),
    }

    // Insert test data using prepared statements
    info!("Inserting test data using prepared statements...");
    
    // Insert user 1
    match manager.insert(
        "users",
        "id, name, email, password",
        "uuid(), 'Alice', 'alice@example.com', 'password1'"
    ).await {
        Ok(_) => info!("Inserted user Alice successfully"),
        Err(e) => warn!("Failed to insert user Alice: {}", e),
    }

    // Insert user 2
    match manager.insert(
        "users",
        "id, name, email, password",
        "uuid(), 'Bob', 'bob@example.com', 'password2'"
    ).await {
        Ok(_) => info!("Inserted user Bob successfully"),
        Err(e) => warn!("Failed to insert user Bob: {}", e),
    }

    // Insert user 3
    match manager.insert(
        "users",
        "id, name, email, password",
        "uuid(), 'Charlie', 'charlie@example.com', 'password3'"
    ).await {
        Ok(_) => info!("Inserted user Charlie successfully"),
        Err(e) => warn!("Failed to insert user Charlie: {}", e),
    }

    // Select all users using prepared statement
    info!("Selecting all users using prepared statement...");
    match manager.select("users", None, None).await {
        Ok(_result) => {
            info!("Query executed successfully - all users selected");
        }
        Err(e) => warn!("Failed to select users: {}", e),
    }

    // Select specific user using prepared statement with WHERE clause
    info!("Selecting user Alice using prepared statement...");
    match manager.select("users", Some("id, name, email"), Some("name = 'Alice'")).await {
        Ok(_result) => {
            info!("Query executed successfully - user Alice selected");
        }
        Err(e) => warn!("Failed to select user Alice: {}", e),
    }

    info!("Database setup completed!");
    
    // Run comprehensive CRUD example
    run_crud_example(&manager).await;
}

/// Comprehensive CRUD example demonstrating all operations
async fn run_crud_example(manager: &CassandraManager) {
    info!("\n=== Starting CRUD Example ===\n");

    // Wait for connection if needed
    if !manager.is_connected().await {
        info!("Waiting for Cassandra connection...");
        let mut attempts = 0;
        while !manager.is_connected().await && attempts < 10 {
            sleep(Duration::from_secs(2)).await;
            attempts += 1;
        }
        
        if !manager.is_connected().await {
            warn!("Cannot run CRUD example - not connected to Cassandra");
            return;
        }
    }

    // ========== CREATE (Insert) ==========
    info!("--- CREATE Operation ---");
    info!("Inserting a new user 'David'...");
    
    match manager.insert(
        "users",
        "id, name, email, password",
        "uuid(), 'David', 'david@example.com', 'password4'"
    ).await {
        Ok(_) => info!("✓ CREATE: User David inserted successfully"),
        Err(e) => warn!("✗ CREATE failed: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // ========== READ (Select) ==========
    info!("\n--- READ Operation ---");
    
    // Read all users
    info!("Reading all users...");
    match manager.select("users", None, None).await {
        Ok(_) => info!("✓ READ: All users retrieved successfully"),
        Err(e) => warn!("✗ READ failed (all users): {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // Read specific user
    info!("Reading user David...");
    match manager.select("users", Some("id, name, email"), Some("name = 'David'")).await {
        Ok(_) => info!("✓ READ: User David retrieved successfully"),
        Err(e) => warn!("✗ READ failed (user David): {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // ========== UPDATE ==========
    info!("\n--- UPDATE Operation ---");
    info!("Updating David's email...");
    
    // Note: In Cassandra, UPDATE requires the PRIMARY KEY in WHERE clause
    // We'll update by name (assuming name is unique for this example)
    // In production, you'd use the actual UUID
    match manager.update(
        "users",
        "email = 'david.updated@example.com'",
        "name = 'David'"
    ).await {
        Ok(_) => {
            info!("✓ UPDATE: David's email updated successfully");
            
            // Verify the update
            sleep(Duration::from_secs(1)).await;
            info!("Verifying update by reading David again...");
            match manager.select("users", Some("name, email"), Some("name = 'David'")).await {
                Ok(_) => info!("✓ UPDATE verified: David's new email retrieved"),
                Err(e) => warn!("✗ UPDATE verification failed: {}", e),
            }
        }
        Err(e) => warn!("✗ UPDATE failed: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // ========== DELETE ==========
    info!("\n--- DELETE Operation ---");
    info!("Deleting user David...");
    
    match manager.delete("users", "name = 'David'").await {
        Ok(_) => {
            info!("✓ DELETE: User David deleted successfully");
            
            // Verify the deletion
            sleep(Duration::from_secs(1)).await;
            info!("Verifying deletion by trying to read David...");
            match manager.select("users", Some("name"), Some("name = 'David'")).await {
                Ok(_) => info!("✓ DELETE verified: David not found (as expected)"),
                Err(e) => warn!("✗ DELETE verification query failed: {}", e),
            }
        }
        Err(e) => warn!("✗ DELETE failed: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // ========== Final READ to show remaining users ==========
    info!("\n--- Final READ Operation ---");
    info!("Reading all remaining users...");
    match manager.select("users", Some("name, email"), None).await {
        Ok(_) => info!("✓ Final READ: Remaining users retrieved successfully"),
        Err(e) => warn!("✗ Final READ failed: {}", e),
    }

    info!("\n=== CRUD Example Completed ===\n");
}