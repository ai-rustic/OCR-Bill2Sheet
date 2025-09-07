use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
    gemini: GeminiConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize)]
struct GeminiConfig {
    api_keys: Vec<String>,
}

fn load_config() -> Config {
    let config_content = fs::read_to_string("config.yaml")
        .expect("Failed to read config.yaml file");
    
    serde_yaml::from_str(&config_content)
        .expect("Failed to parse config.yaml")
}

#[tokio::main]
async fn main() {
    let config = load_config();
    
    println!("Loaded config with {} Gemini API keys", config.gemini.api_keys.len());
    
    let app = Router::new()
        .route("/api/ocr-bill", post(upload_bill_images))
        .layer(CorsLayer::permissive());

    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    println!("Server running on http://{}", bind_address);
    
    axum::serve(listener, app).await.unwrap();
}

async fn upload_bill_images(mut multipart: Multipart) -> Result<Json<Value>, StatusCode> {
    let mut image_index = 1;
    let mut processed_images = Vec::new();
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");
        
        if name == "bill_images" {
            let original_filename = field.file_name().unwrap_or("unknown").to_string();
            let data = field.bytes().await.unwrap();
            
            // Extract file extension from original filename
            let extension = std::path::Path::new(&original_filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("jpg");
            
            // Generate new filename with order: bill_001.jpg, bill_002.jpg, etc.
            let new_filename = format!("bill_{:03}.{}", image_index, extension);
            
            println!("Processing image {}: {} -> {} ({} bytes)", 
                image_index, original_filename, new_filename, data.len());
            
            processed_images.push(json!({
                "original_name": original_filename,
                "new_name": new_filename,
                "order": image_index,
                "size_bytes": data.len(),
                "processing_status": "renamed_successfully"
            }));
            
            image_index += 1;
            
            // TODO: Here you would save the file with new_filename or send to OCR API
            // For now, we just process the data but don't save it
        }
    }
    
    Ok(Json(json!({ 
        "message": "Images processed and renamed successfully",
        "total_images": processed_images.len(),
        "processed_files": processed_images,
        "renamed_image_list": processed_images.iter()
            .map(|img| img["new_name"].as_str().unwrap_or(""))
            .collect::<Vec<&str>>(),
        "processing_timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
