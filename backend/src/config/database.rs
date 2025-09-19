use std::time::Duration;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Error as SqlxError};
use crate::utils::env::{parse_env_u32_with_fallback, parse_env_duration_with_fallback, parse_env_optional_duration_with_fallback};

/// Configuration structure for database connection parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL
    pub database_url: String,
    /// Maximum number of connections in pool (default: 10)
    pub max_connections: u32,
    /// Timeout for acquiring connections (default: 30s)
    pub connection_timeout: Duration,
    /// Idle connection timeout (default: 10min)
    pub idle_timeout: Option<Duration>,
    /// Maximum connection lifetime (default: 30min)
    pub max_lifetime: Option<Duration>,
}

impl DatabaseConfig {
    /// Create a new DatabaseConfig with default values
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)), // 10 minutes
            max_lifetime: Some(Duration::from_secs(1800)), // 30 minutes
        }
    }

    /// Create DatabaseConfig from environment variables
    pub fn from_env() -> Result<Self, DatabaseError> {
        dotenvy::dotenv().ok(); // Load .env file if present

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| DatabaseError::ConfigurationError(
                "DATABASE_URL environment variable is required".to_string()
            ))?;

        // Support both DB_MAX_CONNECTIONS and DATABASE_MAX_CONNECTIONS formats
        let max_connections = parse_env_u32_with_fallback(
            "DATABASE_MAX_CONNECTIONS",
            "DB_MAX_CONNECTIONS",
            10
        );

        // Support both DB_ACQUIRE_TIMEOUT and DATABASE_ACQUIRE_TIMEOUT formats
        let connection_timeout = parse_env_duration_with_fallback(
            "DATABASE_ACQUIRE_TIMEOUT",
            "DB_CONNECTION_TIMEOUT",
            Duration::from_secs(30)
        );

        // Support both DB_IDLE_TIMEOUT and DATABASE_IDLE_TIMEOUT formats
        let idle_timeout = parse_env_optional_duration_with_fallback(
            "DATABASE_IDLE_TIMEOUT",
            "DB_IDLE_TIMEOUT"
        );

        let max_lifetime = parse_env_optional_duration_with_fallback(
            "DATABASE_MAX_LIFETIME",
            "DB_MAX_LIFETIME"
        );

        let config = Self {
            database_url,
            max_connections,
            connection_timeout,
            idle_timeout: idle_timeout.or(Some(Duration::from_secs(600))),
            max_lifetime: max_lifetime.or(Some(Duration::from_secs(1800))),
        };

        // Validate configuration and provide helpful error context
        config.validate().map_err(|e|
            DatabaseError::ConfigurationError(format!(
                "Environment configuration validation failed: {e}. Check your .env file or environment variables."
            ))
        )?;

        Ok(config)
    }

    /// Display configuration details for debugging (excludes sensitive information)
    pub fn display_config(&self) -> String {
        // Mask the password in the database URL for security
        let masked_url = if let Some(at_pos) = self.database_url.find('@') {
            let (prefix, suffix) = self.database_url.split_at(at_pos);
            if let Some(colon_pos) = prefix.rfind(':') {
                format!("{}:***{}", &prefix[..colon_pos], suffix)
            } else {
                self.database_url.clone()
            }
        } else {
            self.database_url.clone()
        };

        format!(
            "DatabaseConfig {{ url: {}, max_connections: {}, connection_timeout: {:?}, idle_timeout: {:?}, max_lifetime: {:?} }}",
            masked_url,
            self.max_connections,
            self.connection_timeout,
            self.idle_timeout,
            self.max_lifetime
        )
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), DatabaseError> {
        // Validate database URL format
        if !self.database_url.starts_with("postgresql://") && !self.database_url.starts_with("postgres://") {
            return Err(DatabaseError::ConfigurationError(
                "database_url must be a valid PostgreSQL URL format".to_string()
            ));
        }

        // Validate max_connections
        if self.max_connections == 0 || self.max_connections > 100 {
            return Err(DatabaseError::ConfigurationError(
                "max_connections must be > 0 and <= 100".to_string()
            ));
        }

        // Validate timeout values are positive
        if self.connection_timeout.is_zero() {
            return Err(DatabaseError::ConfigurationError(
                "connection_timeout must be positive".to_string()
            ));
        }

        if let Some(idle_timeout) = self.idle_timeout {
            if idle_timeout.is_zero() {
                return Err(DatabaseError::ConfigurationError(
                    "idle_timeout must be positive".to_string()
                ));
            }
        }

        if let Some(max_lifetime) = self.max_lifetime {
            if max_lifetime.is_zero() {
                return Err(DatabaseError::ConfigurationError(
                    "max_lifetime must be positive".to_string()
                ));
            }
        }

        Ok(())
    }

    /// Create a PgPool using this configuration
    pub async fn create_pool(&self) -> Result<PgPool, DatabaseError> {
        let mut pool_options = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .acquire_timeout(self.connection_timeout);

        if let Some(idle_timeout) = self.idle_timeout {
            pool_options = pool_options.idle_timeout(idle_timeout);
        }

        if let Some(max_lifetime) = self.max_lifetime {
            pool_options = pool_options.max_lifetime(max_lifetime);
        }

        pool_options
            .connect(&self.database_url)
            .await
            .map_err(DatabaseError::ConnectionError)
    }
}

/// Custom error types for database operations.
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Connection error: {0}")]
    ConnectionError(#[from] SqlxError),

    #[error("Pool error: {0}")]
    PoolError(String),

    #[error("Health check error: {0}")]
    HealthCheckError(SqlxError),
}