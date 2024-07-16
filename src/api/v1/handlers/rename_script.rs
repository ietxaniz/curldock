use crate::api::common::models::Response;
use crate::api::common::{ApiError, ErrorKind};
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameScriptRequest {
    #[serde(rename = "newFullName")]
    pub new_full_name: String,
    #[serde(rename = "oldFullName")]
    pub old_full_name: String,
}

pub async fn rename_script(body: web::Bytes) -> HttpResponse {
    let rename_request: RenameScriptRequest = match serde_json::from_slice(&body) {
        Ok(request) => request,
        Err(e) => {
            return ApiError::new(
                ErrorKind::InvalidInput,
                "rename_script",
                format!("Failed to parse JSON: {}", e),
            )
            .error_response()
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.rename_script(
        &rename_request.old_full_name,
        &rename_request.new_full_name,
    ) {
        Ok(_) => HttpResponse::Ok().json(Response::success("Script renamed successfully")),
        Err(e) => ApiError::from_script_manager_error("rename_script", e).error_response(),
    }
}
