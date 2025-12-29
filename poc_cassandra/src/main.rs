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

    // Create the users table
    let create_table = "
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            name TEXT,
            email TEXT,
            password TEXT
        );
    ";
    
    match manager.query(create_table, &[]).await {
        Ok(_) => info!("Table created successfully"),
        Err(e) => warn!("Failed to create table: {}", e),
    }

    info!("Database setup completed!");
}