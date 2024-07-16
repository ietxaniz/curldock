use crate::api::common::models::Response;
use crate::api::common::{ApiError,ErrorKind};
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError};

pub async fn get_script_details(path: String) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    let remaining_path = path
        .trim_start_matches("/v1/script-details")
        .trim_start_matches('/');
    let parts: Vec<&str> = remaining_path.splitn(2, '/').collect();

    let (path, name) = match parts.len() {
        1 => ("", parts[0]),
        2 => (parts[0], parts[1]),
        _ => {
            return ApiError::new(
                ErrorKind::InvalidInput,
                "get_script_details",
                "Invalid path format".to_string(),
            )
            .error_response()
        }
    };

    match script_manager.get_script_details(path, name) {
        Ok(script_details) => HttpResponse::Ok().json(Response::success(script_details)),
        Err(e) => ApiError::from_script_manager_error("get_script_details", e).error_response(),
    }
}
