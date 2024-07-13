use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn handle_post(body: web::Bytes) -> HttpResponse {
    let received_data: serde_json::Value = serde_json::from_slice(&body).unwrap();
    HttpResponse::Created().json(json!({
        "message": "POST request successful",
        "received": received_data
    }))
}
