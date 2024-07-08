use crate::api::common::models::Response;
use crate::script_manager;
use crate::script_manager::models::ScriptError;
use actix_web::HttpResponse;

pub async fn get_script_details(path: String) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    let remaining_path = path
        .trim_start_matches("/v1/script-details")
        .trim_start_matches('/');
    let parts: Vec<&str> = remaining_path.splitn(2, '/').collect();

    let (path, name) = match parts.len() {
        1 => ("", parts[0]),
        2 => (parts[0], parts[1]),
        _ => {
            return HttpResponse::BadRequest().json(Response::<()>::error(
                "InvalidPath".to_string(),
                "Invalid path format".to_string(),
            ))
        }
    };

    match script_manager.get_script_details(path, name) {
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
                        format!("Invalid path: {}", e),
                    ),
                ),
            };
            status.json(response)
        }
    }
}
