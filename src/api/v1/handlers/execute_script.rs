use actix_web::{web, HttpResponse};
use crate::script_manager::{self, models::ScriptError};
use crate::api::common::models::Response;

pub async fn execute_script(path_info: web::Path<(String, String)>) -> HttpResponse {
    let (path, name) = path_info.into_inner();
    let script_manager = script_manager::get_script_manager();

    match script_manager.execute_script(&path, &name) {
        Ok(execution_result) => HttpResponse::Ok().json(Response::success(execution_result)),
        Err(err) => {
            let (mut status, response) = match err {
                ScriptError::IoError(e) if e.kind() == std::io::ErrorKind::NotFound => (
                    HttpResponse::NotFound(),
                    Response::<()>::error(
                        "ScriptNotFound".to_string(),
                        format!("Script not found: {}", e),
                    ),
                ),
                ScriptError::IoError(e) => (
                    HttpResponse::InternalServerError(),
                    Response::<()>::error(
                        "IoError".to_string(),
                        format!("An IO error occurred: {}", e),
                    ),
                ),
                ScriptError::CurlParseError(e) => (
                    HttpResponse::BadRequest(),
                    Response::<()>::error(
                        "CurlParseError".to_string(),
                        format!("Failed to parse curl command in script: {}", e),
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
            };
            status.json(response)
        }
    }
}
