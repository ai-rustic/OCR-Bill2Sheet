//! Database utility functions for common operations
//!
//! This module contains shared database functionality to eliminate
//! code duplication across different modules.

use sqlx::PgPool;
use crate::config::DatabaseError;

/// Performs a basic database connectivity test using a simple query
///
/// This function is used by both health checks and connection pool validation
/// to ensure consistent behavior across the application.
pub async fn test_database_connectivity(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .is_ok()
}

/// Performs a database connectivity test with error information
///
/// Returns detailed error information for debugging purposes when connectivity fails
pub async fn test_database_connectivity_detailed(pool: &PgPool) -> Result<(), DatabaseError> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(DatabaseError::HealthCheck)?;
    Ok(())
}

/// Pool information retrieval trait for consistent access to pool statistics
///
/// This trait provides a consistent interface for retrieving pool information
/// across different components that need access to connection pool statistics.
pub trait PoolInfo {
    /// Get the current number of connections in the pool
    fn get_pool_size(&self) -> u32;

    /// Get the number of idle connections in the pool
    fn get_idle_connections(&self) -> u32;

    /// Get the maximum allowed connections for the pool
    fn get_max_connections(&self) -> u32;

}

/// Implement PoolInfo for PgPool directly
impl PoolInfo for PgPool {
    fn get_pool_size(&self) -> u32 {
        self.size()
    }

    fn get_idle_connections(&self) -> u32 {
        self.num_idle() as u32
    }

    fn get_max_connections(&self) -> u32 {
        // Note: PgPool doesn't expose max_connections directly
        // This will be overridden by ConnectionPool implementation
        10 // Default fallback
    }
}