use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::response::query_result::QueryResult;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use std::error::Error;
use std::collections::HashMap;
use log::{info, warn, error};
use uuid::Uuid;

// Use the PreparedStatement type from the session's prepare method return type
// We'll infer it from the prepare() method return type

pub struct CassandraManager {
    session: Arc<Mutex<Option<Arc<Session>>>>,
    // Store prepared statements - the type will be inferred from prepare() return type
    prepared_statements: Arc<Mutex<HashMap<String, String>>>, // Store query strings for now, prepare on-demand
    connection_string: String,
    keyspace: Option<String>,
}

impl CassandraManager {
    /// Create a new CassandraManager instance
    /// This will NOT fail even if Cassandra is down
    pub fn new(connection_string: String, keyspace: Option<String>) -> Arc<Self> {
        info!("Initializing CassandraManager with connection: {}", connection_string);
        
        let manager = Arc::new(Self {
            session: Arc::new(Mutex::new(None)),
            prepared_statements: Arc::new(Mutex::new(HashMap::new())),
            connection_string,
            keyspace,
        });

        // Start background connection polling task
        manager.start_connection_polling();

        manager
    }

    /// Start a background task that polls for connection every 5 seconds
    fn start_connection_polling(self: &Arc<Self>) {
        let session: Arc<Mutex<Option<Arc<Session>>>> = Arc::clone(&self.session);
        let connection_string = self.connection_string.clone();
        let _keyspace = self.keyspace.clone();
        let manager = Arc::clone(self);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            // Don't wait for the first tick, try immediately
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            
            loop {
                interval.tick().await;
                
                // Check if we already have a connection
                let has_session = {
                    let s = session.lock().await;
                    s.is_some()
                };

                if !has_session {
                    info!("Attempting to connect to Cassandra at {}...", connection_string);
                    
                    match manager.try_connect_internal().await {
                        Ok(new_session) => {
                            let keyspace_info = if let Some(ref ks) = manager.keyspace {
                                format!(" (keyspace: {})", ks)
                            } else {
                                String::new()
                            };
                            // Use println! to ensure success message is always visible
                            let conn_info = format!("{}{}", connection_string, keyspace_info);
                            println!("\n╔══════════════════════════════════════════════════════════════╗");
                            println!("║  ✓ SUCCESSFULLY CONNECTED TO CASSANDRA!                      ║");
                            println!("║  Connection: {:<50} ║", conn_info);
                            println!("║  Status: Connection established and verified                 ║");
                            println!("║  Ready to execute queries                                    ║");
                            println!("╚══════════════════════════════════════════════════════════════╝\n");
                            info!("✓ Successfully connected to Cassandra at {}{}!", connection_string, keyspace_info);
                            info!("  Connection established and verified. Ready to execute queries.");
                            let mut s = session.lock().await;
                            *s = Some(Arc::new(new_session));
                        }
                        Err(e) => {
                            warn!("Failed to connect to Cassandra: {}. Will retry in 5 seconds...", e);
                        }
                    }
                } else {
                    // Verify connection is still alive by checking if we can query
                    let is_alive = {
                        let s = session.lock().await;
                        if let Some(ref sess) = *s {
                            // Try a simple query to verify connection using prepared statement
                            match sess.prepare("SELECT now() FROM system.local").await {
                                Ok(prepared) => {
                                    // Use empty values - execute_unpaged should accept &[]
                                    match sess.execute_unpaged(&prepared, &[]).await {
                                        Ok(_) => true,
                                        Err(_) => false,
                                    }
                                }
                                Err(_) => false,
                            }
                        } else {
                            false
                        }
                    };

                    if !is_alive {
                        warn!("Cassandra connection lost. Clearing session and prepared statements, will retry...");
                        let mut s = session.lock().await;
                        *s = None;
                        // Clear prepared statements cache when connection is lost
                        let mut stmts = manager.prepared_statements.lock().await;
                        stmts.clear();
                    }
                    // Connection is alive and verified - no need to log every poll cycle
                    // Success is logged when connection is first established
                }
            }
        });
    }

    /// Create keyspace if it doesn't exist
    /// This should be called before trying to use the keyspace
    pub async fn create_keyspace_if_not_exists(
        &self,
        keyspace: &str,
        replication_factor: u32,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{ 'class' : 'SimpleStrategy', 'replication_factor' : {} }}",
            keyspace, replication_factor
        );
        
        info!("Creating keyspace '{}' if it doesn't exist...", keyspace);
        
        // First, connect without keyspace to create it
        let builder = SessionBuilder::new().known_node(&self.connection_string);
        let session = builder.build().await?;
        
        // Create the keyspace using prepared statement
        // Convert String to &str for prepare()
        let query_str: &str = &query;
        let prepared = session.prepare(query_str).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        
        match session.execute_unpaged(&prepared, &[]).await {
            Ok(_) => {
                info!("✓ Keyspace '{}' created or already exists", keyspace);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to create keyspace '{}': {}", keyspace, e);
                Err(Box::new(e) as Box<dyn Error + Send + Sync>)
            }
        }
    }

    /// Attempt to connect to Cassandra (internal method)
    async fn try_connect_internal(&self) -> Result<Session, Box<dyn Error + Send + Sync>> {
        // If keyspace is provided, create it first if it doesn't exist
        if let Some(ref ks) = self.keyspace {
            info!("Checking/creating keyspace '{}' before connecting...", ks);
            // Try to create keyspace, but don't fail if it already exists or if connection fails
            // We'll use a temporary session just for keyspace creation
            if let Err(e) = self.create_keyspace_if_not_exists(ks, 1).await {
                warn!("Could not create keyspace '{}' (may already exist or connection issue): {}", ks, e);
                // Continue anyway - keyspace might already exist
            }
        }
        
        let mut builder = SessionBuilder::new().known_node(&self.connection_string);
        
        // Set keyspace if provided (now that we know it exists or was created)
        if let Some(ref ks) = self.keyspace {
            builder = builder.use_keyspace(ks, true);
            info!("Configuring connection to use keyspace: {}", ks);
        }
        
        info!("Building session connection to {}...", self.connection_string);
        let session = match builder.build().await {
            Ok(s) => {
                info!("Session built successfully, verifying connection...");
                s
            }
            Err(e) => {
                warn!("Failed to build session: {}", e);
                return Err(Box::new(e) as Box<dyn Error + Send + Sync>);
            }
        };

        // Verify connection with a simple query using prepared statement
        match session.prepare("SELECT now() FROM system.local").await {
            Ok(prepared) => {
                match session.execute_unpaged(&prepared, &[]).await {
                    Ok(_) => {
                        info!("Connection verification query executed successfully");
                    }
                    Err(e) => {
                        warn!("Connection verification query failed: {}", e);
                        return Err(Box::new(e) as Box<dyn Error + Send + Sync>);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to prepare verification query: {}", e);
                return Err(Box::new(e) as Box<dyn Error + Send + Sync>);
            }
        }

        Ok(session)
    }

    /// Get the session if available
    /// Returns None if not connected
    pub async fn get_session(&self) -> Option<Arc<Session>> {
        let session = self.session.lock().await;
        session.as_ref().map(|s| Arc::clone(s))
    }

    /// Check if currently connected to Cassandra
    pub async fn is_connected(&self) -> bool {
        let session = self.session.lock().await;
        session.is_some()
    }


    /// Execute a query if connected using prepared statements
    /// Returns an error if not connected or if preparation/execution fails
    /// Fails gracefully - returns error instead of panicking
    pub async fn query(
        &self,
        query: &str,
        _values: &[()], // Placeholder - use execute_prepared for queries with values
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        // Get session for preparation and execution
        let session_arc = {
            let session = self.session.lock().await;
            session.as_ref().map(|s| Arc::clone(s))
        };

        match session_arc {
            Some(s) => {
                // Prepare the statement with graceful error handling
                let prepared = match s.prepare(query).await {
                    Ok(prepared) => prepared,
                    Err(e) => {
                        error!("Failed to prepare statement '{}': {}", query, e);
                        return Err(Box::new(e) as Box<dyn Error + Send + Sync>);
                    }
                };

                // Execute with graceful error handling
                match s.execute_unpaged(&prepared, &[]).await {
                    Ok(result) => {
                        // Track successful queries in cache
                        {
                            let mut stmts = self.prepared_statements.lock().await;
                            stmts.insert(query.to_string(), query.to_string());
                        }
                        Ok(result)
                    }
                    Err(e) => {
                        warn!("Query execution failed for '{}': {}", query, e);
                        // If execution fails, remove from cache to force re-preparation on next attempt
                        {
                            let mut stmts = self.prepared_statements.lock().await;
                            stmts.remove(query);
                        }
                        Err(Box::new(e) as Box<dyn Error + Send + Sync>)
                    }
                }
            }
            None => Err("Not connected to Cassandra".into()),
        }
    }

    /// Execute a prepared statement with values
    /// This is a more advanced method for parameterized queries
    /// Note: For now, this executes with empty values. Full value serialization support can be added later.
    pub async fn execute_prepared(
        &self,
        query: &str,
        _values: &[()], // Placeholder - full value serialization support can be added
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        // Use the same query method which handles preparation gracefully
        self.query(query, &[]).await
    }

    /// Clear the prepared statements cache
    /// Useful when schema changes occur
    pub async fn clear_prepared_statements(&self) {
        let mut stmts = self.prepared_statements.lock().await;
        stmts.clear();
        info!("Cleared prepared statements cache");
    }

    /// Create a table using prepared statement
    /// Fails gracefully if table creation fails
    pub async fn create_table(
        &self,
        table_name: &str,
        columns: &str,
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, columns);
        info!("Creating table '{}' with prepared statement", table_name);
        
        match self.query(&query, &[]).await {
            Ok(result) => {
                info!("Table '{}' created successfully", table_name);
                Ok(result)
            }
            Err(e) => {
                error!("Failed to create table '{}': {}", table_name, e);
                Err(e)
            }
        }
    }

    /// Insert data into a table using prepared statement
    /// Fails gracefully if insertion fails
    pub async fn insert(
        &self,
        table_name: &str,
        columns: &str,
        values: &str,
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let query = format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns, values);
        info!("Inserting into table '{}' with prepared statement", table_name);
        
        match self.query(&query, &[]).await {
            Ok(result) => {
                info!("Data inserted into '{}' successfully", table_name);
                Ok(result)
            }
            Err(e) => {
                warn!("Failed to insert into table '{}': {}", table_name, e);
                Err(e)
            }
        }
    }

    /// Select data from a table using prepared statement
    /// Fails gracefully if query fails
    pub async fn select(
        &self,
        table_name: &str,
        columns: Option<&str>,
        where_clause: Option<&str>,
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let cols = columns.unwrap_or("*");
        let mut query = format!("SELECT {} FROM {}", cols, table_name);
        
        if let Some(where_clause) = where_clause {
            query.push_str(&format!(" WHERE {}", where_clause));
        }
        
        info!("Selecting from table '{}' with prepared statement", table_name);
        
        match self.query(&query, &[]).await {
            Ok(result) => {
                info!("Query executed successfully on '{}'", table_name);
                Ok(result)
            }
            Err(e) => {
                warn!("Failed to select from table '{}': {}", table_name, e);
                Err(e)
            }
        }
    }

    /// Update data in a table using prepared statement
    /// Fails gracefully if update fails
    pub async fn update(
        &self,
        table_name: &str,
        set_clause: &str,
        where_clause: &str,
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let query = format!("UPDATE {} SET {} WHERE {}", table_name, set_clause, where_clause);
        info!("Updating table '{}' with prepared statement", table_name);
        
        match self.query(&query, &[]).await {
            Ok(result) => {
                info!("Data updated in '{}' successfully", table_name);
                Ok(result)
            }
            Err(e) => {
                warn!("Failed to update table '{}': {}", table_name, e);
                Err(e)
            }
        }
    }

    /// Delete data from a table using prepared statement
    /// Fails gracefully if deletion fails
    pub async fn delete(
        &self,
        table_name: &str,
        where_clause: &str,
    ) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let query = format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        info!("Deleting from table '{}' with prepared statement", table_name);
        
        match self.query(&query, &[]).await {
            Ok(result) => {
                info!("Data deleted from '{}' successfully", table_name);
                Ok(result)
            }
            Err(e) => {
                warn!("Failed to delete from table '{}': {}", table_name, e);
                Err(e)
            }
        }
    }

    /// Manually trigger a connection attempt
    pub async fn connect(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Manual connection attempt triggered");
        
        match self.try_connect_internal().await {
            Ok(new_session) => {
                let keyspace_info = if let Some(ref ks) = self.keyspace {
                    format!(" (keyspace: {})", ks)
                } else {
                    String::new()
                };
                // Use println! to ensure success message is always visible
                let conn_info = format!("{}{}", self.connection_string, keyspace_info);
                println!("\n╔══════════════════════════════════════════════════════════════╗");
                println!("║  ✓ SUCCESSFULLY CONNECTED TO CASSANDRA!                      ║");
                println!("║  Connection: {:<50} ║", conn_info);
                println!("║  Status: Connection established and verified                 ║");
                println!("║  Ready to execute queries                                    ║");
                println!("╚══════════════════════════════════════════════════════════════╝\n");
                info!("✓ Successfully connected to Cassandra at {}{}!", self.connection_string, keyspace_info);
                info!("  Connection established and verified. Ready to execute queries.");
                let mut s = self.session.lock().await;
                *s = Some(Arc::new(new_session));
                // Clear prepared statements cache on reconnection to ensure freshness
                let mut stmts = self.prepared_statements.lock().await;
                stmts.clear();
                Ok(())
            }
            Err(e) => {
                warn!("Failed to connect to Cassandra: {}", e);
                Err(e)
            }
        }
    }
}

// User CRUD operations
impl CassandraManager {
    /// Create a new user (INSERT)
    pub async fn create_user(&self, user: &crate::user::User) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let values = format!("{}, '{}', '{}', '{}'", user.id, user.name, user.email, user.password);
        info!("Creating user: {} ({})", user.name, user.email);
        
        self.insert("users", "id, name, email, password", &values).await
    }

    /// Get a user by ID (SELECT)
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<crate::user::User>, Box<dyn Error + Send + Sync>> {
        let query = format!("SELECT id, name, email, password FROM users WHERE id = {}", id);
        info!("Getting user by ID: {}", id);
        
        match self.select("users", Some("id, name, email, password"), Some(&format!("id = {}", id))).await {
            Ok(result) => {
                // Parse result into User - for now return None as parsing requires more work
                // In a real implementation, you'd parse the rows
                info!("User query executed successfully");
                Ok(None) // Placeholder - would need to parse QueryResult rows
            }
            Err(e) => {
                warn!("Failed to get user by ID: {}", e);
                Err(e)
            }
        }
    }

    /// Get a user by email (SELECT)
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<crate::user::User>, Box<dyn Error + Send + Sync>> {
        info!("Getting user by email: {}", email);
        
        match self.select("users", Some("id, name, email, password"), Some(&format!("email = '{}'", email))).await {
            Ok(result) => {
                info!("User query executed successfully");
                Ok(None) // Placeholder - would need to parse QueryResult rows
            }
            Err(e) => {
                warn!("Failed to get user by email: {}", e);
                Err(e)
            }
        }
    }

    /// Get all users (SELECT)
    pub async fn get_all_users(&self) -> Result<Vec<crate::user::User>, Box<dyn Error + Send + Sync>> {
        info!("Getting all users");
        
        match self.select("users", Some("id, name, email, password"), None).await {
            Ok(result) => {
                info!("All users query executed successfully");
                Ok(Vec::new()) // Placeholder - would need to parse QueryResult rows
            }
            Err(e) => {
                warn!("Failed to get all users: {}", e);
                Err(e)
            }
        }
    }

    /// Update a user (UPDATE)
    pub async fn update_user(&self, id: Uuid, name: Option<&str>, email: Option<&str>, password: Option<&str>) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        let mut updates = Vec::new();
        
        if let Some(n) = name {
            updates.push(format!("name = '{}'", n));
        }
        if let Some(e) = email {
            updates.push(format!("email = '{}'", e));
        }
        if let Some(p) = password {
            updates.push(format!("password = '{}'", p));
        }
        
        if updates.is_empty() {
            return Err("No fields to update".into());
        }
        
        let set_clause = updates.join(", ");
        info!("Updating user ID: {}", id);
        
        self.update("users", &set_clause, &format!("id = {}", id)).await
    }

    /// Delete a user by ID (DELETE)
    pub async fn delete_user(&self, id: Uuid) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        info!("Deleting user ID: {}", id);
        self.delete("users", &format!("id = {}", id)).await
    }

    /// Delete a user by email (DELETE)
    pub async fn delete_user_by_email(&self, email: &str) -> Result<QueryResult, Box<dyn Error + Send + Sync>> {
        info!("Deleting user with email: {}", email);
        self.delete("users", &format!("email = '{}'", email)).await
    }
}

