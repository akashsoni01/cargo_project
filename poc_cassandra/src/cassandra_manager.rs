use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::response::query_result::QueryResult;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use std::error::Error;
use std::collections::HashMap;
use log::{info, warn, error};

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
                            info!("Successfully connected to Cassandra!");
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
                }
            }
        });
    }

    /// Attempt to connect to Cassandra (internal method)
    async fn try_connect_internal(&self) -> Result<Session, Box<dyn Error + Send + Sync>> {
        let mut builder = SessionBuilder::new().known_node(&self.connection_string);
        
        // Set keyspace if provided
        if let Some(ref ks) = self.keyspace {
            builder = builder.use_keyspace(ks, true);
        }
        
        let session = builder.build().await?;

        // Use keyspace if provided - set it in the session builder instead
        // The keyspace will be used automatically for queries
        
        // Verify connection with a simple query using prepared statement
        let prepared = session.prepare("SELECT now() FROM system.local").await?;
        session.execute_unpaged(&prepared, &[]).await?;

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

    /// Manually trigger a connection attempt
    pub async fn connect(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Manual connection attempt triggered");
        
        match self.try_connect_internal().await {
            Ok(new_session) => {
                info!("Successfully connected to Cassandra!");
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

