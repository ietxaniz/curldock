use actix_web::HttpResponse;
use serde_json::json;

pub async fn handle_error() -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "message": "Error occurred"
    }))
}
