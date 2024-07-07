use crate::api::common::models::Response;
use crate::script_manager::models::ScriptError;
use crate::script_manager::ScriptManager;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

pub async fn get_script_details(
    script_manager: web::Data<Arc<ScriptManager>>,
    path_info: web::Path<(String, String)>,
) -> HttpResponse {
    let (path, name) = path_info.into_inner();
    match script_manager.get_script_details(&path, &name) {
        Ok(script_details) => HttpResponse::Ok().json(Response::success(script_details)),
        Err(err) => {
            let (mut status, response) = match err {
                ScriptError::IoError(e) => (
                    HttpResponse::NotFound(),
                    Response::<()>::error(
                        "IoError".to_string(),
                        format!("Script not found: {}", e),
                    ),
                ),
                ScriptError::CurlParseError(e) => (
                    HttpResponse::BadRequest(),
                    Response::<()>::error(
                        "CurlParseError".to_string(),
                        format!("Invalid curl command in script: {}", e),
                    ),
                ),
                ScriptError::ExecutionError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "ExecutionError".to_string(),
                        format!("Error executing script: {}", e),
                    ),
                ),
                ScriptError::CommandGenerationError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "CommandGenerationError".to_string(),
                        format!("Error generating curl command: {}", e),
                    ),
                ),
                ScriptError::OutputCaptureError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "OutputCaptureError".to_string(),
                        format!("Error capturing curl output: {}", e),
                    ),
                ),
                ScriptError::OutputParseError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "OutputParseError".to_string(),
                        format!("Error parsing curl output: {}", e),
                    ),
                ),
                ScriptError::CommandExecutionError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "CommandExecutionError".to_string(),
                        format!("Error during command execution: {}", e),
                    ),
                ),
                ScriptError::ScriptNotFound(e) => (
                    HttpResponse::NotFound(),
                    Response::<()>::error(
                        "ScriptNotFound".to_string(),
                        format!("Script not found: {}", e),
                    ),
                ),
                ScriptError::InvalidPath(e) => (
                    HttpResponse::BadRequest(),
                    Response::<()>::error(
                        "InvalidPath".to_string(),
                        format!("Invalid path:  {}", e),
                    ),
                ),
            };
            status.json(response)
        }
    }
}
