mod api;
mod config;
mod models;
mod services;
mod utils;

use axum::{
    middleware,
    routing::get,
    Router,
};
use tracing::{info, error, warn};
use std::net::SocketAddr;
use std::time::Duration;

use config::{ConnectionPool, DatabaseConfig};
use api::{get_health, get_health_detail, error_handling_middleware, timeout_middleware, not_found_handler};

/// Type alias for the application state shared across all Axum handlers
/// This makes it clear what state is available to handlers and improves maintainability
type AppState = ConnectionPool;

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    info!("Starting OCR_Bill2Sheet backend server...");

    // Initialize the database connection pool with enhanced startup validation
    let pool = initialize_database_connection().await;

    // Validate database connectivity and schema access
    if let Err(e) = validate_database_startup(&pool).await {
        error!("Database startup validation failed: {}", e);
        std::process::exit(1);
    }

    info!("Database connection and validation completed successfully");

    // Create the Axum router with health endpoints, middleware layers, and connection pool state
    // The AppState (ConnectionPool) is shared across all handlers via Axum's State system
    let app = Router::new()
        .route("/health", get(get_health))
        .route("/health/detail", get(get_health_detail))
        .fallback(not_found_handler)
        .layer(middleware::from_fn(request_logging_middleware))
        .layer(middleware::from_fn(error_handling_middleware))
        .layer(middleware::from_fn(timeout_middleware))
        .with_state(pool.clone()); // Clone is cheap for Arc-wrapped connection pool

    // Set up the server address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Starting server on {}", addr);

    // Create the TCP listener
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Server listening on {}", addr);
            listener
        }
        Err(e) => {
            error!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    // Start the server
    info!("Server startup complete, ready to accept requests");
    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}

/// Request logging middleware for comprehensive request/response tracking
///
/// This middleware logs all incoming requests with method, URI, response status,
/// response time, and client information. It integrates with the error handling
/// middleware to provide complete observability for API requests.
async fn request_logging_middleware(
    request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use std::time::Instant;
    use uuid::Uuid;

    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version_str = format!("{:?}", request.version());

    // Generate a unique request ID for tracing
    let request_id = Uuid::new_v4().to_string();

    // Extract client IP if available from headers
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Extract user agent if available
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    info!(
        method = %method,
        uri = %uri,
        version = %version_str,
        client_ip = %client_ip,
        user_agent = %user_agent,
        request_id = %request_id,
        "Incoming request"
    );

    // Call the next middleware/handler in the chain
    let response = next.run(request).await;

    // Calculate response time
    let response_time = start_time.elapsed();
    let response_time_ms = response_time.as_millis();
    let status = response.status();

    // Log request completion with appropriate level based on status
    if status.is_server_error() {
        error!(
            method = %method,
            uri = %uri,
            status = %status,
            response_time_ms = %response_time_ms,
            client_ip = %client_ip,
            request_id = %request_id,
            "Request completed with server error"
        );
    } else if status.is_client_error() {
        warn!(
            method = %method,
            uri = %uri,
            status = %status,
            response_time_ms = %response_time_ms,
            client_ip = %client_ip,
            request_id = %request_id,
            "Request completed with client error"
        );
    } else {
        info!(
            method = %method,
            uri = %uri,
            status = %status,
            response_time_ms = %response_time_ms,
            client_ip = %client_ip,
            request_id = %request_id,
            "Request completed successfully"
        );
    }

    response
}

/// Initialize database connection pool with retry logic and comprehensive error handling
async fn initialize_database_connection() -> ConnectionPool {
    info!("Initializing database connection pool...");

    // First attempt: direct connection
    match ConnectionPool::from_env().await {
        Ok(pool) => {
            info!("Database connection pool initialized successfully on first attempt");
            return pool;
        }
        Err(e) => {
            warn!("Initial database connection failed, attempting with retry logic: {}", e);
        }
    }

    // Second attempt: use retry logic for resilience
    let config = match DatabaseConfig::from_env() {
        Ok(config) => {
            info!("Database configuration loaded: {}", config.display_config());
            config
        }
        Err(e) => {
            error!("Failed to load database configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Attempt connection with retry logic
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(2);

    match ConnectionPool::new_with_retry(config, MAX_RETRIES, RETRY_DELAY).await {
        Ok(pool) => {
            info!("Database connection pool initialized successfully with retry logic");
            pool
        }
        Err(e) => {
            error!("Failed to initialize database connection pool after {} retries: {}", MAX_RETRIES, e);
            error!("Please check:");
            error!("  1. PostgreSQL server is running");
            error!("  2. Database 'bill_ocr' exists");
            error!("  3. Connection credentials are correct");
            error!("  4. Network connectivity to database server");
            std::process::exit(1);
        }
    }
}

/// Validate database connectivity and basic operations during startup
async fn validate_database_startup(pool: &ConnectionPool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Performing database startup validation...");

    // Test basic connectivity
    pool.health_check().await.map_err(|e| {
        error!("Database health check failed: {}", e);
        e
    })?;
    info!("✓ Database connectivity verified");

    // Log connection pool status
    info!("✓ Connection pool status: {} active connections, {} idle connections",
          pool.pool_size(), pool.idle_connections());

    // Verify we can query basic PostgreSQL information
    let version_result = sqlx::query_scalar::<_, String>(
        "SELECT version() as pg_version"
    )
    .fetch_one(pool.pool())
    .await;

    match version_result {
        Ok(version) => {
            info!("✓ PostgreSQL version: {}", version.split_whitespace().take(2).collect::<Vec<_>>().join(" "));
        }
        Err(e) => {
            warn!("Could not retrieve PostgreSQL version: {}", e);
        }
    }

    // Verify database name matches expected 'bill_ocr'
    let db_name_result = sqlx::query_scalar::<_, String>(
        "SELECT current_database() as db_name"
    )
    .fetch_one(pool.pool())
    .await;

    match db_name_result {
        Ok(db_name) => {
            if db_name == "bill_ocr" {
                info!("✓ Connected to correct database: {}", db_name);
            } else {
                warn!("Connected to database '{}', expected 'bill_ocr'", db_name);
            }
        }
        Err(e) => {
            warn!("Could not verify database name: {}", e);
        }
    }

    info!("Database startup validation completed successfully");
    Ok(())
}
