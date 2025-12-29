use std::error::Error;
use log::{info, warn};
use tokio::time::{sleep, Duration};

/// Common trait for all repositories
/// Ensures consistent initialization and error handling across repositories
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

    // ========== Common Helper Methods (Default Implementations) ==========

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

    /// Check if repository is ready (connected and initialized)
    /// This combines connection check with a basic health check
    /// 
    /// # Returns
    /// * `true` if repository is ready, `false` otherwise
    async fn is_ready(&self) -> bool {
        if !self.is_connected().await {
            return false;
        }
        
        // Additional checks can be added here
        // For now, just check connection status
        true
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
        
        Ok(())
    }

    /// Initialize with validation
    /// First validates configuration, then initializes
    /// 
    /// # Returns
    /// * `Ok(())` if initialization succeeds, `Err` if validation or initialization fails
    async fn initialize_with_validation(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Validate configuration first
        self.validate_config()?;
        
        info!("Configuration validated for keyspace: {}", self.keyspace_name());
        
        // Then initialize
        self.initialize().await
    }
}

