mod api;
mod config;
mod errors;
mod models;
mod services;
mod state;
mod utils;

use axum::{
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, error, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;

use config::{ConnectionPool, DatabaseConfig, UploadConfig, ServerConfig};
use state::AppState;
use api::{
    get_health, get_health_detail, error_handling_middleware, timeout_middleware, not_found_handler,
    get_all_bills, get_bill_by_id, create_bill, update_bill, delete_bill, search_bills, get_bills_count,
    upload_images_sse
};


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

    // Initialize server configuration
    let server_config = match ServerConfig::from_env() {
        Ok(config) => {
            info!("Server configuration loaded: {}", config.display_config());
            config
        }
        Err(e) => {
            error!("Failed to load server configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize upload configuration
    let upload_config = match UploadConfig::from_env() {
        Ok(config) => {
            info!("Upload configuration loaded: max_file_size_bytes={}, max_image_count={}",
                  config.max_file_size_bytes, config.max_image_count);
            Arc::new(config)
        }
        Err(e) => {
            error!("Failed to load upload configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create event broadcaster for SSE
    let (event_broadcaster, _) = broadcast::channel(1000);
    info!("Event broadcaster initialized with buffer size: 1000");

    // Create unified application state
    let app_state = AppState {
        pool: pool.clone(),
        upload_config: upload_config.clone(),
        event_broadcaster,
    };

    // Create router with unified state
    let app = Router::new()
        // Health endpoints
        .route("/health", get(get_health))
        .route("/health/detail", get(get_health_detail))
        // Bill endpoints with /api/ prefix
        .route("/api/bills", get(get_all_bills).post(create_bill))
        .route("/api/bills/search", get(search_bills))
        .route("/api/bills/count", get(get_bills_count))
        .route("/api/bills/{id}", get(get_bill_by_id).put(update_bill).delete(delete_bill))
        // OCR endpoints
        .route("/api/ocr", post(upload_images_sse))
        .fallback(not_found_handler)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50MB total request limit
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .layer(middleware::from_fn(request_logging_middleware))
        .layer(middleware::from_fn(error_handling_middleware))
        .layer(middleware::from_fn(timeout_middleware))
        .with_state(app_state);

    // Set up the server address from configuration
    let addr = server_config.socket_addr();
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
