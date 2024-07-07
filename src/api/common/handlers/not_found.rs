use actix_web::HttpResponse;
use crate::api::common::models::Response;

pub async fn not_found() -> HttpResponse {
    let error_response = Response::<()>::error(
        "NotFound".to_string(),
        "The requested resource was not found.".to_string(),
    );
    HttpResponse::NotFound().json(error_response)
}
