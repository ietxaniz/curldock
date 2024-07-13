use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn handle_put(body: web::Bytes) -> HttpResponse {
    let updated_data: serde_json::Value = serde_json::from_slice(&body).unwrap();
    HttpResponse::Ok().json(json!({
        "message": "PUT request successful",
        "updated": updated_data
    }))
}
