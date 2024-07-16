use crate::api::common::models::Response;
use crate::api::common::{ApiError,ErrorKind};
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError};

pub async fn execute_script(path: String) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    let remaining_path = match path.strip_prefix("/v1/execute") {
        Some(stripped) => stripped.strip_prefix('/').unwrap_or(""),
        None => {
            return ApiError::new(
                ErrorKind::InvalidInput,
                "execute_script",
                "Routes error: path should have started with /v1/execute".to_string(),
            )
            .error_response()
        }
    };

    let parts: Vec<&str> = remaining_path.splitn(2, '/').collect();

    let (script_path, script_name) = match parts.len() {
        1 => ("", parts[0]),
        2 => (parts[0], parts[1]),
        _ => {
            return ApiError::new(
                ErrorKind::InvalidInput,
                "execute_script",
                "Invalid path format for script execution".to_string(),
            )
            .error_response()
        }
    };

    match script_manager.execute_script(script_path, script_name) {
        Ok(execution_result) => HttpResponse::Ok().json(Response::success(execution_result)),
        Err(e) => ApiError::from_script_manager_error("execute_script", e).error_response(),
    }
}
