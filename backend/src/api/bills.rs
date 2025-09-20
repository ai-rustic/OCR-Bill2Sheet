//! Bill API endpoints
//!
//! This module contains all REST API endpoints for bill management.
//! Each handler follows the same pattern: extract state, create service,
//! call service method, wrap response in ApiResponse format.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::{
    api::{ApiError, ApiResponse},
    config::ConnectionPool,
    models::{Bill, CreateBill},
    services::bill_service::BillService,
};

/// GET /api/bills endpoint handler
///
/// Returns all bills from the database in a paginated format.
/// Uses the BillService to fetch all bills and wraps the response
/// in the standard ApiResponse format.
///
/// # Returns
/// - 200 OK with list of bills on success
/// - 500 Internal Server Error on database error
pub async fn get_all_bills(
    State(pool): State<ConnectionPool>,
) -> impl IntoResponse {
    // Create bill service with the connection pool
    let bill_service = BillService::new(pool.pool().clone());

    // Fetch all bills from the database
    match bill_service.get_all_bills().await {
        Ok(bills) => {
            let response = ApiResponse::success(bills);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(api_error) => {
            let response: ApiResponse<Vec<Bill>> = match api_error {
                ApiError::NotFound(msg) => ApiResponse::error(format!("Bills not found: {msg}")),
                ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to fetch bills: {msg}")),
                ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// GET /api/bills/{id} endpoint handler
///
/// Returns a specific bill by its ID.
/// Uses the BillService to fetch the bill and handles the case
/// where the bill might not exist.
///
/// # Parameters
/// - `id`: The bill ID from the URL path
///
/// # Returns
/// - 200 OK with bill data if found
/// - 404 Not Found if bill doesn't exist
/// - 500 Internal Server Error on database error
pub async fn get_bill_by_id(
    State(pool): State<ConnectionPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // Create bill service with the connection pool
    let bill_service = BillService::new(pool.pool().clone());

    // Fetch the bill by ID
    match bill_service.get_bill_by_id(id).await {
        Ok(Some(bill)) => {
            let response = ApiResponse::success(bill);
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response: ApiResponse<Bill> = ApiResponse::error(format!("Bill with ID {id} not found"));
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(api_error) => {
            let response: ApiResponse<Bill> = match api_error {
                ApiError::NotFound(msg) => ApiResponse::error(format!("Bill not found: {msg}")),
                ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to fetch bill: {msg}")),
                ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// POST /api/bills endpoint handler
///
/// Creates a new bill with the provided data.
/// Uses the BillService to create the bill and returns
/// the created bill with its assigned ID.
///
/// # Parameters
/// - `create_bill`: The bill data from the request body
///
/// # Returns
/// - 201 Created with the created bill data
/// - 400 Bad Request on validation error
/// - 500 Internal Server Error on database error
pub async fn create_bill(
    State(pool): State<ConnectionPool>,
    Json(create_bill): Json<CreateBill>,
) -> impl IntoResponse {
    // Create bill service with the connection pool
    let bill_service = BillService::new(pool.pool().clone());

    // Create the new bill
    match bill_service.create_bill(create_bill).await {
        Ok(bill) => {
            let response = ApiResponse::success(bill);
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(api_error) => {
            let response: ApiResponse<Bill> = match api_error {
                ApiError::NotFound(msg) => ApiResponse::error(format!("Resource not found: {msg}")),
                ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to create bill: {msg}")),
                ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

/// PUT /api/bills/{id} endpoint handler
///
/// Updates an existing bill with the provided data.
/// Uses the BillService to update the bill and handles
/// the case where the bill might not exist.
///
/// # Parameters
/// - `id`: The bill ID from the URL path
/// - `update_bill`: The updated bill data from the request body
///
/// # Returns
/// - 200 OK with the updated bill data if successful
/// - 404 Not Found if bill doesn't exist
/// - 400 Bad Request on validation error
/// - 500 Internal Server Error on database error
pub async fn update_bill(
    State(pool): State<ConnectionPool>,
    Path(id): Path<i32>,
    Json(update_bill): Json<CreateBill>,
) -> impl IntoResponse {
    // Create bill service with the connection pool
    let bill_service = BillService::new(pool.pool().clone());

    // Update the bill
    match bill_service.update_bill(id, update_bill).await {
        Ok(Some(bill)) => {
            let response = ApiResponse::success(bill);
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response: ApiResponse<Bill> = ApiResponse::error(format!("Bill with ID {id} not found"));
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(api_error) => {
            let response: ApiResponse<Bill> = match api_error {
                ApiError::NotFound(msg) => ApiResponse::error(format!("Bill not found: {msg}")),
                ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to update bill: {msg}")),
                ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

/// DELETE /api/bills/{id} endpoint handler
///
/// Deletes a bill by its ID.
/// Uses the BillService to delete the bill and returns
/// a success confirmation or error message.
///
/// # Parameters
/// - `id`: The bill ID from the URL path
///
/// # Returns
/// - 200 OK with success message if deleted
/// - 404 Not Found if bill doesn't exist
/// - 500 Internal Server Error on database error
pub async fn delete_bill(
    State(pool): State<ConnectionPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // Create bill service with the connection pool
    let bill_service = BillService::new(pool.pool().clone());

    // Delete the bill
    match bill_service.delete_bill(id).await {
        Ok(true) => {
            let response = ApiResponse::success(format!("Bill with ID {id} deleted successfully"));
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(false) => {
            let response: ApiResponse<String> = ApiResponse::error(format!("Bill with ID {id} not found"));
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(api_error) => {
            let response: ApiResponse<String> = match api_error {
                ApiError::NotFound(msg) => ApiResponse::error(format!("Bill not found: {msg}")),
                ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to delete bill: {msg}")),
                ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

/// Search query parameters for bill search endpoint
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    /// Search pattern for invoice number (partial match)
    #[serde(rename = "q")]
    pub query: Option<String>,

    /// Alternative parameter name for query
    #[serde(rename = "invoice")]
    pub invoice: Option<String>,
}

/// GET /api/bills/search endpoint handler
///
/// Searches bills by invoice number using a pattern match.
/// Supports query parameters 'q' or 'invoice' for the search term.
/// Uses the BillService to search bills with ILIKE pattern matching.
///
/// # Query Parameters
/// - `q` or `invoice`: Search pattern for invoice number
///
/// # Returns
/// - 200 OK with list of matching bills
/// - 400 Bad Request if no search query provided
/// - 500 Internal Server Error on database error
pub async fn search_bills(
    State(pool): State<ConnectionPool>,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse {
    // Extract search query from either 'q' or 'invoice' parameter
    let search_query = params.query.or(params.invoice);

    match search_query {
        Some(query) if !query.trim().is_empty() => {
            // Create bill service with the connection pool
            let bill_service = BillService::new(pool.pool().clone());

            // Search bills by invoice pattern
            match bill_service.search_bills_by_invoice(&query).await {
                Ok(bills) => {
                    let response = ApiResponse::success(bills);
                    (StatusCode::OK, Json(response)).into_response()
                }
                Err(api_error) => {
                    let response: ApiResponse<Vec<Bill>> = match api_error {
                        ApiError::NotFound(msg) => ApiResponse::error(format!("Bills not found: {msg}")),
                        ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to search bills: {msg}")),
                        ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                        ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
                }
            }
        }
        _ => {
            let response: ApiResponse<Vec<Bill>> = ApiResponse::error("Search query parameter 'q' or 'invoice' is required".to_string());
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

/// GET /api/bills/count endpoint handler
///
/// Returns the total count of bills in the database.
/// Uses the BillService to get the aggregate count and
/// wraps it in the standard ApiResponse format.
///
/// # Returns
/// - 200 OK with count number on success
/// - 500 Internal Server Error on database error
pub async fn get_bills_count(
    State(pool): State<ConnectionPool>,
) -> impl IntoResponse {
    // Create bill service with the connection pool
    let bill_service = BillService::new(pool.pool().clone());

    // Get the total count of bills
    match bill_service.get_bills_count().await {
        Ok(count) => {
            let response = ApiResponse::success(count);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(api_error) => {
            let response: ApiResponse<i64> = match api_error {
                ApiError::NotFound(msg) => ApiResponse::error(format!("Count not available: {msg}")),
                ApiError::InternalServerError(msg) => ApiResponse::error(format!("Failed to get bills count: {msg}")),
                ApiError::BadRequest(msg) => ApiResponse::error(format!("Bad request: {msg}")),
                ApiError::ServiceUnavailable(msg) => ApiResponse::error(format!("Service unavailable: {msg}")),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}