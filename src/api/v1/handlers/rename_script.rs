// File: v1/handlers/rename_script.rs

use actix_web::{web, HttpResponse};
use crate::script_manager::{self, models::ScriptError};
use crate::api::common::models::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameScriptRequest {
    pub old_name: String,
    pub new_name: String,
    pub old_path: String,
    pub new_path: String,
}

pub async fn rename_script(rename_request: web::Json<RenameScriptRequest>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    match script_manager.rename_script(
        &rename_request.old_path,
        &rename_request.old_name,
        &rename_request.new_path,
        &rename_request.new_name
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
                },
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