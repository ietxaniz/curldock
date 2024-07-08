use crate::api::common::models::Response;
use crate::script_manager::{self, models::ScriptError};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateFolderRequest {
    pub path: String,
}

pub async fn create_folder(body: web::Bytes) -> HttpResponse {
    let folder_request: CreateFolderRequest = match serde_json::from_slice(&body) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(Response::<()>::error(
                "InvalidJSON".to_string(),
                format!("Failed to parse JSON: {}", e),
            ))
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.create_folder(&folder_request.path) {
        Ok(_) => HttpResponse::Ok().json(Response::success("Folder created successfully")),
        Err(err) => {
            let (mut status, response) = match err {
                ScriptError::IoError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "IoError".to_string(),
                        format!("An IO error occurred: {}", e),
                    ),
                ),
                ScriptError::InvalidPath(e) => (
                    HttpResponse::BadRequest(),
                    Response::<()>::error(
                        "InvalidPath".to_string(),
                        format!("Invalid path: {}", e),
                    ),
                ),
                _ => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "UnexpectedError".to_string(),
                        "An unexpected error occurred".to_string(),
                    ),
                ),
            };
            status.json(response)
        }
    }
}
