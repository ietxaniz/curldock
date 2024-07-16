use crate::api::common::models::Response;
use crate::api::common::{ApiError,ErrorKind};
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateFolderRequest {
    pub path: String,
}

pub async fn create_folder(body: web::Bytes) -> HttpResponse {
    let folder_request: CreateFolderRequest = match serde_json::from_slice(&body) {
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

    match script_manager.create_folder(&folder_request.path) {
        Ok(_) => HttpResponse::Ok().json(Response::success("Folder created successfully")),
        Err(e) => ApiError::from_script_manager_error("create_folder", e).error_response(),
    }
}
