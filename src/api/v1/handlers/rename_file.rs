use crate::api::common::models::Response;
use crate::api::common::{ApiError, ErrorKind};
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameFileRequest {
    #[serde(rename = "oldPath")]
    pub old_path: String,
    #[serde(rename = "newPath")]
    pub new_path: String,
}

pub async fn rename_file(body: web::Bytes) -> HttpResponse {
    let request: RenameFileRequest = match serde_json::from_slice(&body) {
        Ok(request) => request,
        Err(e) => {
            return ApiError::new(
                ErrorKind::InvalidInput,
                "create_folder",
                format!("Failed to parse JSON: {}", e),
            )
            .error_response()
        }
    };
    let script_manager = script_manager::get_script_manager();

    match script_manager.rename_file(&request.old_path, &request.new_path) {
        Ok(_) => HttpResponse::Ok().json(Response::success("File renamed successfully")),
        Err(e) => ApiError::from_script_manager_error("rename_file", e).error_response(),
    }
}
