use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}};
use std::fs;
use base64::{Engine as _, engine::general_purpose};
use mime_guess::from_path;

#[derive(Debug, Deserialize, Clone)]
struct Config {
    server: ServerConfig,
    gemini: GeminiConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Deserialize, Clone)]
struct GeminiConfig {
    api_keys: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillMeta {
    bill_number: String,
    seller: String,
    buyer: String,
    seller_tax_code: String,
    buyer_tax_code: String,
    bill_date: String,
    total_amount: String,
    vat_amount: String,
    payment_method: String,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LineItem {
    no: i32,
    product_name: String,
    quantity: String,
    unit: String,
    unit_price: String,
    subtotal: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BillData {
    bill_meta: BillMeta,
    line_items: Vec<LineItem>,
    notes: String,
}

#[derive(Debug, Serialize)]
struct GeminiInlineData {
    mime_type: String,
    data: String,
}

#[derive(Debug, Serialize)]
struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_data: Option<GeminiInlineData>,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiGenerationConfig {
    response_mime_type: String,
    response_schema: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    generation_config: GeminiGenerationConfig,
}

fn load_config() -> Config {
    let config_content = fs::read_to_string("config.yaml")
        .expect("Failed to read config.yaml file");
    
    serde_yaml::from_str(&config_content)
        .expect("Failed to parse config.yaml")
}

fn image_to_base64(image_data: &[u8]) -> String {
    general_purpose::STANDARD.encode(image_data)
}

fn get_mime_type(filename: &str) -> String {
    from_path(filename)
        .first_or_octet_stream()
        .to_string()
}

fn create_response_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "bill_meta": {
                "type": "object",
                "properties": {
                    "bill_number": {"type": "string"},
                    "seller": {"type": "string"},
                    "buyer": {"type": "string"},
                    "seller_tax_code": {"type": "string"},
                    "buyer_tax_code": {"type": "string"},
                    "bill_date": {"type": "string"},
                    "total_amount": {"type": "string"},
                    "vat_amount": {"type": "string"},
                    "payment_method": {"type": "string"},
                    "address": {"type": "string"}
                }
            },
            "line_items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "no": {"type": "integer"},
                        "product_name": {"type": "string"},
                        "quantity": {"type": "string"},
                        "unit": {"type": "string"},
                        "unit_price": {"type": "string"},
                        "subtotal": {"type": "string"}
                    }
                }
            },
            "notes": {"type": "string"}
        }
    })
}

async fn call_gemini_ocr(images_data: Vec<(String, Vec<u8>)>, api_key: &str) -> Result<BillData, Box<dyn std::error::Error>> {
    let mut parts = vec![
        GeminiPart {
            text: Some("The following images are multi-page bills. Please extract standardized data according to the structured output below.".to_string()),
            inline_data: None,
        }
    ];

    for (filename, image_data) in images_data {
        let mime_type = get_mime_type(&filename);
        let base64_data = image_to_base64(&image_data);
        
        parts.push(GeminiPart {
            text: None,
            inline_data: Some(GeminiInlineData {
                mime_type,
                data: base64_data,
            }),
        });
    }

    let request = GeminiRequest {
        contents: vec![GeminiContent { parts }],
        generation_config: GeminiGenerationConfig {
            response_mime_type: "application/json".to_string(),
            response_schema: create_response_schema(),
        },
    };

    let client = reqwest::Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}", api_key);
    
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Gemini API Response: {}", response_text);

    let response_json: Value = serde_json::from_str(&response_text)?;
    
    if let Some(candidates) = response_json.get("candidates") {
        if let Some(candidate) = candidates.as_array().and_then(|arr| arr.first()) {
            if let Some(content) = candidate.get("content") {
                if let Some(parts) = content.get("parts") {
                    if let Some(part) = parts.as_array().and_then(|arr| arr.first()) {
                        if let Some(text) = part.get("text") {
                            let bill_data: BillData = serde_json::from_str(text.as_str().unwrap())?;
                            return Ok(bill_data);
                        }
                    }
                }
            }
        }
    }

    Err("Failed to parse Gemini response".into())
}


#[tokio::main]
async fn main() {
    let config = load_config();
    
    println!("Loaded config with {} Gemini API keys", config.gemini.api_keys.len());
    
    let app = Router::new()
        .route("/api/ocr-bill", post(upload_bill_images))
        .fallback_service(
            ServeDir::new("../frontend/out")
                .not_found_service(ServeFile::new("../frontend/out/index.html"))
        )
        .with_state(config.clone())
        .layer(DefaultBodyLimit::max(200 * 1024 * 1024)) // 200MB for up to 20 images of 10MB each
        .layer(CorsLayer::permissive());

    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    println!("Server running on http://{}", bind_address);
    
    axum::serve(listener, app).await.unwrap();
}

async fn upload_bill_images(
    State(config): State<Config>,
    mut multipart: Multipart,
) -> Result<Json<Value>, StatusCode> {
    let mut image_index = 1;
    let mut images_data = Vec::new();
    
    loop {
        match multipart.next_field().await {
            Ok(Some(field)) => {
                let name = field.name().unwrap_or("").to_string();
                println!("Processing multipart field: {}", name);
                
                if name == "bill_images" {
                    let original_filename = field.file_name().unwrap_or("unknown").to_string();
                    
                    match field.bytes().await {
                        Ok(data) => {
                            if data.is_empty() {
                                println!("Warning: Empty file data for field '{}'", name);
                                continue;
                            }
                            
                            let extension = std::path::Path::new(&original_filename)
                                .extension()
                                .and_then(|ext| ext.to_str())
                                .unwrap_or("jpg");
                            
                            let new_filename = format!("bill_{:03}.{}", image_index, extension);
                            
                            println!("Processing image {}: {} -> {} ({} bytes)", 
                                image_index, original_filename, new_filename, data.len());
                            
                            images_data.push((new_filename, data.to_vec()));
                            image_index += 1;
                        }
                        Err(e) => {
                            let error_msg = e.to_string();
                            if error_msg.contains("length limit exceeded") {
                                println!("File size limit exceeded: {}", error_msg);
                                return Err(StatusCode::PAYLOAD_TOO_LARGE);
                            } else {
                                println!("Error reading field bytes: {}", error_msg);
                                return Err(StatusCode::BAD_REQUEST);
                            }
                        }
                    }
                } else {
                    // Handle other fields by consuming them
                    match field.text().await {
                        Ok(_) => {
                            println!("Processed text field: {}", name);
                        }
                        Err(e) => {
                            println!("Error reading text field '{}': {}", name, e);
                            return Err(StatusCode::BAD_REQUEST);
                        }
                    }
                }
            }
            Ok(None) => {
                // No more fields
                break;
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("length limit exceeded") {
                    println!("Request size limit exceeded: {}", error_msg);
                    return Err(StatusCode::PAYLOAD_TOO_LARGE);
                } else {
                    println!("Error getting next field: {}", error_msg);
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        }
    }
    
    if images_data.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if config.gemini.api_keys.is_empty() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let api_key = &config.gemini.api_keys[0];
    
    match call_gemini_ocr(images_data, api_key).await {
        Ok(bill_data) => {
            println!("Successfully processed bill: {}", bill_data.bill_meta.bill_number);
            
            Ok(Json(json!({
                "message": "Bill processed successfully",
                "bill_data": bill_data,
                "processing_timestamp": chrono::Utc::now().to_rfc3339()
            })))
        }
        Err(e) => {
            println!("Error processing bill: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
