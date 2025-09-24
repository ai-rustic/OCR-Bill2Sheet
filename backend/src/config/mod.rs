pub mod database;
pub mod gemini_config;
pub mod server_config;
pub mod upload_config;

pub use database::{DatabaseConfig, DatabaseError};
use sqlx::PgPool;
pub use upload_config::UploadConfig;
// pub use gemini_config::{GeminiConfig, GeminiConfigError};
use crate::utils::database::{PoolInfo, test_database_connectivity_detailed};
pub use server_config::{ServerConfig, ServerConfigError};

/// Wrapper around SQLx PgPool for database connection management.
#[derive(Debug, Clone)]
pub struct ConnectionPool {
    /// SQLx PostgreSQL connection pool
    pool: PgPool,
    /// Configuration used to create pool
    config: DatabaseConfig,
}

impl ConnectionPool {
    /// Create a new ConnectionPool from DatabaseConfig with initialization validation
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        // Validate configuration before creating pool
        config.validate().map_err(|e| {
            DatabaseError::Configuration(format!("Pool initialization failed: {e}"))
        })?;

        // Create the connection pool
        let pool = config.create_pool().await?;

        let connection_pool = Self { pool, config };

        // Validate initial connection and warm up pool
        connection_pool.initialize_pool().await?;

        Ok(connection_pool)
    }

    /// Create ConnectionPool from environment variables with full initialization
    pub async fn from_env() -> Result<Self, DatabaseError> {
        let config = DatabaseConfig::from_env().map_err(|e| {
            DatabaseError::Configuration(format!("Environment configuration failed: {e}"))
        })?;

        Self::new(config).await
    }

    /// Initialize and warm up the connection pool
    async fn initialize_pool(&self) -> Result<(), DatabaseError> {
        // Test initial connection
        self.health_check()
            .await
            .map_err(|e| DatabaseError::Pool(format!("Initial connection test failed: {e}")))?;

        // Warm up pool by acquiring and releasing connections
        // This ensures we have working connections available immediately
        let warmup_connections = std::cmp::min(3, self.config.max_connections);

        for i in 0..warmup_connections {
            match self.pool.acquire().await {
                Ok(_conn) => {
                    // Connection acquired successfully, it will be automatically returned to pool
                    tracing::debug!("Pool warmup connection {} successful", i + 1);
                }
                Err(e) => {
                    tracing::warn!("Pool warmup connection {} failed: {}", i + 1, e);
                    // Don't fail initialization for warmup issues, just log
                }
            }
        }

        tracing::info!(
            "Connection pool initialized successfully - {} max connections, {} warmup connections tested",
            self.config.max_connections,
            warmup_connections
        );

        Ok(())
    }

    /// Create ConnectionPool with retry logic for better initialization reliability
    pub async fn new_with_retry(
        config: DatabaseConfig,
        max_retries: u32,
        retry_delay: std::time::Duration,
    ) -> Result<Self, DatabaseError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match Self::new(config.clone()).await {
                Ok(pool) => {
                    if attempt > 0 {
                        tracing::info!(
                            "Connection pool initialized successfully after {} retries",
                            attempt
                        );
                    }
                    return Ok(pool);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        tracing::warn!(
                            "Connection pool initialization attempt {} failed, retrying in {:?}: {}",
                            attempt + 1,
                            retry_delay,
                            last_error.as_ref().unwrap()
                        );
                        tokio::time::sleep(retry_delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            DatabaseError::Pool("Unknown error during pool initialization".to_string())
        }))
    }

    /// Get reference to the underlying PgPool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get reference to the configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Check if the pool is healthy by executing a simple query
    pub async fn health_check(&self) -> Result<(), DatabaseError> {
        test_database_connectivity_detailed(&self.pool).await
    }

    /// Get current pool size (number of connections)
    pub fn pool_size(&self) -> u32 {
        self.get_pool_size()
    }

    /// Get number of idle connections in the pool
    pub fn idle_connections(&self) -> u32 {
        self.get_idle_connections()
    }

    /// Close the connection pool
    pub async fn close(&self) {
        self.pool.close().await;
    }
}

/// Allow ConnectionPool to be used as Axum State
impl From<ConnectionPool> for PgPool {
    fn from(pool: ConnectionPool) -> Self {
        pool.pool
    }
}

/// Allow easy conversion to PgPool for direct SQLx usage
impl AsRef<PgPool> for ConnectionPool {
    fn as_ref(&self) -> &PgPool {
        &self.pool
    }
}

/// Implement PoolInfo trait for ConnectionPool
impl PoolInfo for ConnectionPool {
    fn get_pool_size(&self) -> u32 {
        self.pool.size()
    }

    fn get_idle_connections(&self) -> u32 {
        self.pool.num_idle() as u32
    }

    fn get_max_connections(&self) -> u32 {
        self.config.max_connections
    }
}
