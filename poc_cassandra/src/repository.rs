use std::error::Error;
use log::{info, warn};
use tokio::time::{sleep, Duration};

/// Common trait for all repositories
/// Ensures consistent initialization and error handling across repositories
/// Provides default implementations for common operations
pub trait Repository: Send + Sync {
    /// Initialize the repository (create keyspace and table if needed)
    /// Each repository handles its own initialization independently
    /// Connection failures in one repository do not affect others
    async fn initialize(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Check if the repository is connected to Cassandra
    /// Returns true if connected, false otherwise
    async fn is_connected(&self) -> bool;
    
    /// Get the keyspace name used by this repository
    fn keyspace_name(&self) -> &str;

    // ========== Connection Management (Default Implementations) ==========

    /// Wait for connection with timeout
    /// This is a common pattern used by all repositories during initialization
    /// 
    /// # Arguments
    /// * `max_attempts` - Maximum number of attempts to wait for connection
    /// * `interval_secs` - Seconds to wait between attempts
    /// 
    /// # Returns
    /// * `true` if connected, `false` if timeout reached
    async fn wait_for_connection(
        &self,
        max_attempts: u32,
        interval_secs: u64,
    ) -> bool {
        let mut attempts = 0;
        
        while !self.is_connected().await && attempts < max_attempts {
            sleep(Duration::from_secs(interval_secs)).await;
            attempts += 1;
        }
        
        self.is_connected().await
    }

    /// Wait for connection with default timeout (30 attempts, 1 second interval)
    /// Convenience method for common initialization pattern
    async fn wait_for_connection_default(&self) -> bool {
        self.wait_for_connection(30, 1).await
    }

    /// Ensure connection before operation
    /// Returns error if not connected, useful for operations that require connection
    /// 
    /// # Returns
    /// * `Ok(())` if connected, `Err` with message if not connected
    async fn ensure_connected(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if !self.is_connected().await {
            return Err(format!(
                "Repository '{}' is not connected to Cassandra",
                self.keyspace_name()
            ).into());
        }
        Ok(())
    }

    // ========== Retry Logic (Default Implementations) ==========

    /// Retry an operation with exponential backoff
    /// Useful for transient failures that might succeed on retry
    /// 
    /// # Arguments
    /// * `operation` - Async closure that returns Result
    /// * `max_retries` - Maximum number of retry attempts
    /// * `initial_delay_ms` - Initial delay in milliseconds (doubles each retry)
    /// 
    /// # Returns
    /// * `Ok(T)` if operation succeeds, `Err(E)` if all retries fail
    async fn retry_operation<F, Fut, T, E>(
        operation: F,
        max_retries: u32,
        initial_delay_ms: u64,
    ) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let mut delay_ms = initial_delay_ms;
        
        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt < max_retries {
                        warn!("Operation failed (attempt {}/{}), retrying in {}ms...", 
                            attempt + 1, max_retries + 1, delay_ms);
                        sleep(Duration::from_millis(delay_ms)).await;
                        delay_ms *= 2; // Exponential backoff
                    } else {
                        warn!("Operation failed after {} attempts", max_retries + 1);
                        return Err(e);
                    }
                }
            }
        }
        
        unreachable!()
    }

    /// Retry an operation with fixed delay between retries
    /// Simpler than exponential backoff for cases where fixed delay is preferred
    /// 
    /// # Arguments
    /// * `operation` - Async closure that returns Result
    /// * `max_retries` - Maximum number of retry attempts
    /// * `delay_ms` - Fixed delay in milliseconds between retries
    /// 
    /// # Returns
    /// * `Ok(T)` if operation succeeds, `Err(E)` if all retries fail
    async fn retry_operation_fixed<F, Fut, T, E>(
        operation: F,
        max_retries: u32,
        delay_ms: u64,
    ) -> Result<T, E>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt < max_retries {
                        warn!("Operation failed (attempt {}/{}), retrying in {}ms...", 
                            attempt + 1, max_retries + 1, delay_ms);
                        sleep(Duration::from_millis(delay_ms)).await;
                    } else {
                        warn!("Operation failed after {} attempts", max_retries + 1);
                        return Err(e);
                    }
                }
            }
        }
        
        unreachable!()
    }

    // ========== Health Checks (Default Implementations) ==========

    /// Check if repository is ready (connected and initialized)
    /// This combines connection check with a basic health check
    /// 
    /// # Returns
    /// * `true` if repository is ready, `false` otherwise
    async fn is_ready(&self) -> bool {
        self.is_connected().await
    }

    /// Perform a health check on the repository
    /// Returns detailed health status information
    /// 
    /// # Returns
    /// * `(healthy: bool, message: String)` - Health status and description
    async fn health_check(&self) -> (bool, String) {
        let connected = self.is_connected().await;
        let ready = self.is_ready().await;
        let keyspace = self.keyspace_name();
        
        if connected && ready {
            (true, format!("Repository '{}' is healthy and ready", keyspace))
        } else if connected {
            (false, format!("Repository '{}' is connected but not ready", keyspace))
        } else {
            (false, format!("Repository '{}' is not connected", keyspace))
        }
    }

    /// Get repository status information
    /// Returns a formatted string with connection and keyspace information
    async fn status(&self) -> String {
        let connected = self.is_connected().await;
        let ready = self.is_ready().await;
        let keyspace = self.keyspace_name();
        
        format!(
            "Repository Status:\n  Keyspace: {}\n  Connected: {}\n  Ready: {}",
            keyspace,
            if connected { "✓ Yes" } else { "✗ No" },
            if ready { "✓ Yes" } else { "✗ No" }
        )
    }

    /// Log repository status
    /// Convenience method to log current repository status
    async fn log_status(&self) {
        info!("{}", self.status().await);
    }

    /// Log health check results
    /// Convenience method to log health check information
    async fn log_health(&self) {
        let (healthy, message) = self.health_check().await;
        if healthy {
            info!("✓ {}", message);
        } else {
            warn!("✗ {}", message);
        }
    }

    // ========== Configuration Validation (Default Implementations) ==========

    /// Validate repository configuration
    /// Checks if keyspace name is valid (non-empty, valid format)
    /// 
    /// # Returns
    /// * `Ok(())` if valid, `Err` with description if invalid
    fn validate_config(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let keyspace = self.keyspace_name();
        
        if keyspace.is_empty() {
            return Err("Keyspace name cannot be empty".into());
        }
        
        // Basic validation: keyspace name should be alphanumeric with underscores
        if !keyspace.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!("Invalid keyspace name format: '{}'. Must be alphanumeric with underscores only.", keyspace).into());
        }
        
        // Check length (Cassandra has limits)
        if keyspace.len() > 48 {
            return Err(format!("Keyspace name '{}' is too long (max 48 characters)", keyspace).into());
        }
        
        Ok(())
    }

    /// Initialize with validation
    /// First validates configuration, then initializes
    /// 
    /// # Returns
    /// * `Ok(())` if initialization succeeds, `Err` if validation or initialization fails
    async fn initialize_with_validation(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Validate configuration first
        match self.validate_config() {
            Ok(_) => {
                info!("Configuration validated for keyspace: {}", self.keyspace_name());
            }
            Err(e) => {
                warn!("Configuration validation failed: {}", e);
                return Err(e);
            }
        }
        
        // Then initialize
        self.initialize().await
    }

    /// Initialize with retry logic
    /// Attempts initialization with exponential backoff on failure
    /// 
    /// # Arguments
    /// * `max_retries` - Maximum number of retry attempts (default: 3)
    /// * `initial_delay_ms` - Initial delay in milliseconds (default: 1000)
    /// 
    /// # Returns
    /// * `Ok(())` if initialization succeeds, `Err` if all retries fail
    async fn initialize_with_retry(
        &self,
        max_retries: u32,
        initial_delay_ms: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let operation = || self.initialize();
        Self::retry_operation(operation, max_retries, initial_delay_ms).await
    }

    /// Initialize with validation and retry
    /// Combines validation, initialization, and retry logic
    /// 
    /// # Arguments
    /// * `max_retries` - Maximum number of retry attempts (default: 3)
    /// * `initial_delay_ms` - Initial delay in milliseconds (default: 1000)
    /// 
    /// # Returns
    /// * `Ok(())` if initialization succeeds, `Err` if validation or all retries fail
    async fn initialize_with_validation_and_retry(
        &self,
        max_retries: u32,
        initial_delay_ms: u64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Validate first
        self.validate_config()?;
        
        // Then initialize with retry
        self.initialize_with_retry(max_retries, initial_delay_ms).await
    }
}

