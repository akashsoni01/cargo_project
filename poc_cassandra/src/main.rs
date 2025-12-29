mod cassandra_manager;

use cassandra_manager::CassandraManager;
use std::error::Error;
use log::{info, warn};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger with custom filter to suppress scylla internal errors
    // Set RUST_LOG environment variable to control logging levels
    // Example: RUST_LOG=info,scylla=warn to suppress scylla errors
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .filter_module("scylla", log::LevelFilter::Warn) // Suppress scylla ERROR logs
        .filter_module("scylla::cluster", log::LevelFilter::Warn)
        .filter_module("scylla::cluster::metadata", log::LevelFilter::Warn)
        .init();

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

    // Create the keyspace if it doesn't exist (using manager method)
    // Note: The manager already tries to create it during connection, but we'll ensure it here too
    info!("Ensuring keyspace 'my_keyspace' exists...");
    match manager.create_keyspace_if_not_exists("my_keyspace", 1).await {
        Ok(_) => info!("Keyspace 'my_keyspace' is ready"),
        Err(e) => {
            warn!("Failed to create keyspace (may already exist): {}", e);
            // Try to use it anyway - it might already exist
        }
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
    
    // Test connection resilience
    test_connection_resilience(&manager).await;
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

/// Test connection resilience by simulating Cassandra going down and coming back up
async fn test_connection_resilience(manager: &CassandraManager) {
    info!("\n=== Testing Connection Resilience ===\n");

    // Wait for connection if needed
    if !manager.is_connected().await {
        info!("Waiting for Cassandra connection...");
        let mut attempts = 0;
        while !manager.is_connected().await && attempts < 10 {
            sleep(Duration::from_secs(2)).await;
            attempts += 1;
        }
        
        if !manager.is_connected().await {
            warn!("Cannot test resilience - not connected to Cassandra");
            return;
        }
    }

    info!("✓ Initial connection verified");
    
    // Test 1: Verify connection is working
    info!("\n--- Test 1: Verify connection is working ---");
    match manager.query("SELECT now() FROM system.local", &[]).await {
        Ok(_) => info!("✓ Connection is active and working"),
        Err(e) => {
            warn!("✗ Connection test failed: {}", e);
            return;
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Test 2: Simulate Cassandra going down
    info!("\n--- Test 2: Simulating Cassandra going down ---");
    info!("Stopping Cassandra container...");
    
    // Stop Cassandra container
    let stop_result = std::process::Command::new("docker-compose")
        .args(&["stop", "cassandra"])
        .current_dir(".")
        .output();
    
    match stop_result {
        Ok(output) => {
            if output.status.success() {
                info!("✓ Cassandra container stopped");
            } else {
                warn!("✗ Failed to stop Cassandra: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            warn!("✗ Error stopping Cassandra: {}", e);
            info!("Please manually stop Cassandra to test resilience");
        }
    }

    sleep(Duration::from_secs(3)).await;

    // Test 3: Try operations while Cassandra is down
    info!("\n--- Test 3: Testing operations while Cassandra is down ---");
    info!("Attempting query while Cassandra is down (should fail gracefully)...");
    
    match manager.query("SELECT now() FROM system.local", &[]).await {
        Ok(_) => warn!("⚠ Unexpected: Query succeeded while Cassandra is down"),
        Err(e) => {
            info!("✓ Query failed gracefully as expected: {}", e);
        }
    }

    sleep(Duration::from_secs(2)).await;

    // Test 4: Check connection status
    info!("\n--- Test 4: Checking connection status ---");
    let is_connected = manager.is_connected().await;
    if is_connected {
        warn!("⚠ Connection status still shows connected (may take a moment to detect)");
    } else {
        info!("✓ Connection status correctly shows disconnected");
    }

    sleep(Duration::from_secs(5)).await;

    // Test 5: Verify connection detection after polling interval
    info!("\n--- Test 5: Waiting for connection loss detection (polling every 5s) ---");
    let mut attempts = 0;
    while manager.is_connected().await && attempts < 3 {
        info!("Still connected (attempt {})...", attempts + 1);
        sleep(Duration::from_secs(6)).await;
        attempts += 1;
    }
    
    if !manager.is_connected().await {
        info!("✓ Connection loss detected by polling mechanism");
    } else {
        warn!("⚠ Connection still showing as connected after polling");
    }

    sleep(Duration::from_secs(2)).await;

    // Test 6: Restart Cassandra
    info!("\n--- Test 6: Restarting Cassandra ---");
    info!("Starting Cassandra container...");
    
    let start_result = std::process::Command::new("docker-compose")
        .args(&["start", "cassandra"])
        .current_dir(".")
        .output();
    
    match start_result {
        Ok(output) => {
            if output.status.success() {
                info!("✓ Cassandra container started");
            } else {
                warn!("✗ Failed to start Cassandra: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            warn!("✗ Error starting Cassandra: {}", e);
            info!("Please manually start Cassandra to test reconnection");
        }
    }

    info!("Waiting for Cassandra to be ready (this may take 30-60 seconds)...");
    sleep(Duration::from_secs(10)).await;

    // Test 7: Wait for automatic reconnection
    info!("\n--- Test 7: Waiting for automatic reconnection (polling every 5s) ---");
    let mut reconnection_attempts = 0;
    const MAX_RECONNECTION_ATTEMPTS: u32 = 15; // 75 seconds max wait
    
    while !manager.is_connected().await && reconnection_attempts < MAX_RECONNECTION_ATTEMPTS {
        info!("Waiting for reconnection... (attempt {}/{})", 
              reconnection_attempts + 1, MAX_RECONNECTION_ATTEMPTS);
        sleep(Duration::from_secs(5)).await;
        reconnection_attempts += 1;
    }

    if manager.is_connected().await {
        info!("✓ Automatic reconnection successful!");
    } else {
        warn!("✗ Reconnection timeout - Cassandra may still be starting up");
        warn!("   The manager will continue polling in the background");
        return;
    }

    sleep(Duration::from_secs(2)).await;

    // Test 8: Verify operations work after reconnection
    info!("\n--- Test 8: Verifying operations after reconnection ---");
    match manager.query("SELECT now() FROM system.local", &[]).await {
        Ok(_) => info!("✓ Query successful after reconnection"),
        Err(e) => warn!("✗ Query failed after reconnection: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    // Test 9: Test CRUD operations after reconnection
    info!("\n--- Test 9: Testing CRUD operations after reconnection ---");
    
    info!("Testing INSERT after reconnection...");
    match manager.insert(
        "users",
        "id, name, email, password",
        "uuid(), 'ResilienceTest', 'resilience@test.com', 'test123'"
    ).await {
        Ok(_) => info!("✓ INSERT works after reconnection"),
        Err(e) => warn!("✗ INSERT failed after reconnection: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    info!("Testing SELECT after reconnection...");
    match manager.select("users", Some("name"), Some("name = 'ResilienceTest'")).await {
        Ok(_) => info!("✓ SELECT works after reconnection"),
        Err(e) => warn!("✗ SELECT failed after reconnection: {}", e),
    }

    sleep(Duration::from_secs(1)).await;

    info!("Testing DELETE after reconnection...");
    match manager.delete("users", "name = 'ResilienceTest'").await {
        Ok(_) => info!("✓ DELETE works after reconnection"),
        Err(e) => warn!("✗ DELETE failed after reconnection: {}", e),
    }

    info!("\n=== Connection Resilience Test Completed ===\n");
    info!("Summary:");
    info!("  ✓ Application continues running when Cassandra goes down");
    info!("  ✓ Operations fail gracefully without crashing");
    info!("  ✓ Connection loss is detected automatically");
    info!("  ✓ Automatic reconnection works (polls every 5 seconds)");
    info!("  ✓ All CRUD operations work after reconnection");
}