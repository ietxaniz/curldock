use crate::api::common::models::Response;
use crate::script_manager::{self, models::ScriptError};
use actix_web::HttpResponse;

pub async fn execute_script(path: String) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    let remaining_path = match path.strip_prefix("/v1/execute") {
        Some(stripped) => stripped.strip_prefix('/').unwrap_or(""),
        None => {
            return HttpResponse::InternalServerError()
                .body("Routes error: path should have started with /v1/execute")
        }
    };
    let parts: Vec<&str> = remaining_path.splitn(2, '/').collect();

    let (script_path, script_name) = match parts.len() {
        1 => ("", parts[0]),
        2 => (parts[0], parts[1]),
        _ => {
            return HttpResponse::BadRequest().json(Response::<()>::error(
                "InvalidPath".to_string(),
                "Invalid path format for script execution".to_string(),
            ))
        }
    };

    match script_manager.execute_script(script_path, script_name) {
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
