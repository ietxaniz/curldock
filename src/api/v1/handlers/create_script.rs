use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager::{self, models::ScriptDetailsCreate};
use actix_web::{web, HttpResponse, ResponseError};

pub async fn create_script(body: web::Bytes) -> HttpResponse {
    let body_str = String::from_utf8_lossy(&body);
    println!("Received body: {}", body_str);

    let script_details: ScriptDetailsCreate = match serde_json::from_slice(&body) {
        Ok(details) => details,
        Err(e) => {
            return ApiError::new("create_script",
                format!("Failed to parse JSON: {}", e),
            )
            .error_response()
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.create_script(script_details) {
        Ok(created_script) => HttpResponse::Created().json(Response::success(created_script)),
        Err(e) => ApiError::from_debug_error("create_script", e).error_response(),
    }
}
