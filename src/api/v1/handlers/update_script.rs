use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager::{self, models::ScriptDetailsCreate};
use actix_web::{web, HttpResponse, ResponseError};

pub async fn update_script(body: web::Bytes) -> HttpResponse {
    let script_details: ScriptDetailsCreate = match serde_json::from_slice(&body) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(Response::<()>::error(
                "InvalidJSON".to_string(),
                format!("Failed to parse JSON: {}", e),
            ))
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.update_script(script_details) {
        Ok(updated_script) => HttpResponse::Ok().json(Response::success(updated_script)),
        Err(err) => {
            return ApiError::from_script_manager_error("update_script", err).error_response()
        }
    }
}
