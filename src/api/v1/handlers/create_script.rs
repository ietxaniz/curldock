use actix_web::{web, HttpResponse};
use crate::script_manager::{self, models::{ScriptDetailsCreate, ScriptError}};
use crate::api::common::models::Response;

pub async fn create_script(script_details: web::Json<ScriptDetailsCreate>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    match script_manager.create_script(script_details.into_inner()) {
        Ok(created_script) => HttpResponse::Created().json(Response::success(created_script)),
        Err(err) => {
            let (mut status, response) = match err {
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
