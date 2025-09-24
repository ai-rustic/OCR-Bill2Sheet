use crate::{config::ConnectionPool, services::health::HealthService};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};

/// GET /health endpoint handler
///
/// Returns the current health status of the database connection.
/// Returns 200 OK for healthy status, 503 Service Unavailable for unhealthy status.
///
/// Uses Axum State extraction to access the shared ConnectionPool instance.
pub async fn get_health(State(pool): State<ConnectionPool>) -> impl IntoResponse {
    // Create health service with the connection pool
    let health_service = HealthService::new(pool.pool().clone(), pool.config().clone());

    // Perform health check
    let health_status = health_service.check_health().await;

    // Determine response status code based on health
    let status_code = if health_status.is_healthy() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    // Return response with appropriate status code and JSON body
    (status_code, Json(health_status))
}

/// GET /health/detail endpoint handler
///
/// Returns detailed information about database connectivity and pool status.
/// Always returns 200 OK with detailed health information regardless of database status.
///
/// Uses Axum State extraction to access the shared ConnectionPool instance.
pub async fn get_health_detail(State(pool): State<ConnectionPool>) -> impl IntoResponse {
    // Create health service with the connection pool
    let health_service = HealthService::new(pool.pool().clone(), pool.config().clone());

    // Perform detailed health check with graceful error handling
    let detailed_health_status = health_service.check_detailed_health_safe().await;

    // Always return 200 OK for detailed health endpoint as per contract
    (StatusCode::OK, Json(detailed_health_status))
}
