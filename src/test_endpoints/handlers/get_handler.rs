use actix_web::HttpResponse;
use serde_json::json;

pub async fn handle_get() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "GET request successful",
        "data": { "id": 1, "name": "Test Item" }
    }))
}
