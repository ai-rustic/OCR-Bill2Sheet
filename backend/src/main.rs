use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/ocr-bill", post(upload_bill_images))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2011").await.unwrap();
    println!("Server running on http://0.0.0.0:2011");
    
    axum::serve(listener, app).await.unwrap();
}

async fn upload_bill_images(mut multipart: Multipart) -> Result<Json<Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");
        
        if name == "bill_images" {
            let _data = field.bytes().await.unwrap();
        }
    }
    
    Ok(Json(json!({ "message": "OK" })))
}
