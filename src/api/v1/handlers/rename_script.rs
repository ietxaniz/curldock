// File: v1/handlers/rename_script.rs

use crate::api::common::models::Response;
use crate::script_manager::{self, models::ScriptError};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameScriptRequest {
    #[serde(rename = "oldName")]
    pub old_name: String,
    #[serde(rename = "newName")]
    pub new_name: String,
    #[serde(rename = "oldPath")]
    pub old_path: String,
    #[serde(rename = "newPath")]
    pub new_path: String,
}

pub async fn rename_script(body: web::Bytes) -> HttpResponse {
    let rename_request: RenameScriptRequest = match serde_json::from_slice(&body) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(Response::<()>::error(
                "InvalidJSON".to_string(),
                format!("Failed to parse JSON: {}", e),
            ))
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.rename_script(
        &rename_request.old_path,
        &rename_request.old_name,
        &rename_request.new_path,
        &rename_request.new_name,
    ) {
        Ok(renamed_script) => HttpResponse::Ok().json(Response::success(renamed_script)),
        Err(err) => {
            let (mut status, response) = match err {
                ScriptError::IoError(e) => {
                    if e.kind() == std::io::ErrorKind::AlreadyExists {
                        (
                            HttpResponse::Conflict(),
                            Response::<()>::error(
                                "ScriptAlreadyExists".to_string(),
                                format!("A script with the new name already exists in the destination path: {}", e),
                            ),
                        )
                    } else {
                        (
                            HttpResponse::InternalServerError(),
                            Response::<()>::error(
                                "IoError".to_string(),
                                format!("An IO error occurred: {}", e),
                            ),
                        )
                    }
                }
                ScriptError::ScriptNotFound(e) => (
                    HttpResponse::NotFound(),
                    Response::<()>::error(
                        "ScriptNotFound".to_string(),
                        format!("Script not found: {}", e),
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
