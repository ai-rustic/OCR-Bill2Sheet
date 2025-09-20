use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::config::database::DatabaseConfig;
use crate::utils::database::{test_database_connectivity, PoolInfo};

/// Database connectivity health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Status indicator: "healthy" or "unhealthy"
    pub status: String,
    /// Whether database queries can be executed
    pub database_accessible: bool,
    /// Current number of connections in the pool
    pub pool_size: u32,
    /// Timestamp when health check was performed
    pub timestamp: DateTime<Utc>,
}

/// Detailed health check response with additional pool and configuration information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedHealthStatus {
    /// Status indicator: "healthy" or "unhealthy"
    pub status: String,
    /// Whether database queries can be executed
    pub database_accessible: bool,
    /// Current number of connections in the pool
    pub pool_size: u32,
    /// Timestamp when health check was performed
    pub timestamp: DateTime<Utc>,
    /// Maximum allowed connections in pool
    pub max_connections: u32,
    /// Number of idle connections
    pub idle_connections: u32,
    /// Database configuration details
    pub configuration: ConfigurationDetails,
}

/// Configuration details for detailed health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationDetails {
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Idle timeout in seconds (if configured)
    pub idle_timeout_seconds: Option<u64>,
    /// Maximum lifetime in seconds (if configured)
    pub max_lifetime_seconds: Option<u64>,
}

impl HealthStatus {
    /// Create a status from database accessibility
    pub fn from_database_check(database_accessible: bool, pool_size: u32) -> Self {
        Self {
            status: if database_accessible { "healthy" } else { "unhealthy" }.to_string(),
            database_accessible,
            pool_size,
            timestamp: Utc::now(),
        }
    }


    /// Check if the health status is healthy
    pub fn is_healthy(&self) -> bool {
        self.database_accessible && self.status == "healthy"
    }
}

impl DetailedHealthStatus {
    /// Create a detailed status from database accessibility
    pub fn from_database_check(
        database_accessible: bool,
        pool_size: u32,
        max_connections: u32,
        idle_connections: u32,
        config: &DatabaseConfig,
    ) -> Self {
        Self {
            status: if database_accessible { "healthy" } else { "unhealthy" }.to_string(),
            database_accessible,
            pool_size,
            timestamp: Utc::now(),
            max_connections,
            idle_connections,
            configuration: ConfigurationDetails {
                connection_timeout_seconds: config.connection_timeout.as_secs(),
                idle_timeout_seconds: config.idle_timeout.map(|d| d.as_secs()),
                max_lifetime_seconds: config.max_lifetime.map(|d| d.as_secs()),
            },
        }
    }

}

/// Health check service implementation
pub struct HealthService {
    pool: PgPool,
    config: DatabaseConfig,
}

impl HealthService {
    /// Create a new health service
    pub fn new(pool: PgPool, config: DatabaseConfig) -> Self {
        Self { pool, config }
    }

    /// Perform a basic health check
    pub async fn check_health(&self) -> HealthStatus {
        let pool_size = self.pool.get_pool_size();
        let database_accessible = test_database_connectivity(&self.pool).await;
        HealthStatus::from_database_check(database_accessible, pool_size)
    }


    /// Perform a detailed health check with graceful error handling
    pub async fn check_detailed_health_safe(&self) -> DetailedHealthStatus {
        let pool_size = self.pool.get_pool_size();
        let max_connections = self.pool.get_max_connections();
        let idle_connections = self.pool.get_idle_connections();

        let database_accessible = test_database_connectivity(&self.pool).await;

        DetailedHealthStatus::from_database_check(
            database_accessible,
            pool_size,
            max_connections,
            idle_connections,
            &self.config,
        )
    }

}

