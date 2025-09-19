use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::config::database::{DatabaseConfig, DatabaseError};
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

    /// Create a new healthy status
    pub fn healthy(pool_size: u32) -> Self {
        Self::from_database_check(true, pool_size)
    }

    /// Create a new unhealthy status
    pub fn unhealthy(pool_size: u32) -> Self {
        Self::from_database_check(false, pool_size)
    }

    /// Validate the health status consistency
    pub fn validate(&self) -> Result<(), DatabaseError> {
        // Status must be "healthy" when database_accessible is true
        if self.database_accessible && self.status != "healthy" {
            return Err(DatabaseError::ConfigurationError(
                "status must be 'healthy' when database_accessible is true".to_string()
            ));
        }

        // Status must be "unhealthy" when database_accessible is false
        if !self.database_accessible && self.status != "unhealthy" {
            return Err(DatabaseError::ConfigurationError(
                "status must be 'unhealthy' when database_accessible is false".to_string()
            ));
        }

        Ok(())
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

    /// Create a new detailed healthy status
    pub fn healthy(
        pool_size: u32,
        max_connections: u32,
        idle_connections: u32,
        config: &DatabaseConfig,
    ) -> Self {
        Self::from_database_check(true, pool_size, max_connections, idle_connections, config)
    }

    /// Create a new detailed unhealthy status
    pub fn unhealthy(
        pool_size: u32,
        max_connections: u32,
        idle_connections: u32,
        config: &DatabaseConfig,
    ) -> Self {
        Self::from_database_check(false, pool_size, max_connections, idle_connections, config)
    }

    /// Check if the detailed health status is healthy
    pub fn is_healthy(&self) -> bool {
        self.database_accessible && self.status == "healthy"
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

    /// Perform a detailed health check with connection pool information
    pub async fn check_detailed_health(&self) -> Result<DetailedHealthStatus, DatabaseError> {
        let pool_size = self.pool.get_pool_size();
        let max_connections = self.pool.get_max_connections();
        let idle_connections = self.pool.get_idle_connections();

        let database_accessible = test_database_connectivity(&self.pool).await;

        let status = DetailedHealthStatus::from_database_check(
            database_accessible,
            pool_size,
            max_connections,
            idle_connections,
            &self.config,
        );

        if !database_accessible {
            return Err(DatabaseError::HealthCheckError(
                sqlx::Error::PoolClosed
            ));
        }

        Ok(status)
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

    /// Get the current pool size
    pub fn get_pool_size(&self) -> u32 {
        self.pool.get_pool_size()
    }

    /// Get pool idle connections count
    pub fn get_idle_connections(&self) -> u32 {
        self.pool.get_idle_connections()
    }

    /// Get the maximum connections configured for the pool
    pub fn get_max_connections(&self) -> u32 {
        self.pool.get_max_connections()
    }

    /// Get a reference to the database configuration
    pub fn get_config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Check if the pool has available connections
    pub fn has_available_connections(&self) -> bool {
        self.pool.has_available_connections()
    }

    /// Get pool utilization as a percentage (0.0 to 1.0)
    pub fn get_pool_utilization(&self) -> f32 {
        self.pool.get_pool_utilization()
    }
}

