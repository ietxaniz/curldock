use crate::api::common::models::Response;
use crate::script_manager::{
    self,
    models::{ScriptDetailsCreate, ScriptError},
};
use actix_web::{web, HttpResponse};

pub async fn create_script(body: web::Bytes) -> HttpResponse {
    let body_str = String::from_utf8_lossy(&body);
    println!("Received body: {}", body_str);

    let script_details: ScriptDetailsCreate = match serde_json::from_slice(&body) {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::BadRequest().json(Response::<()>::error(
                "InvalidJSON".to_string(),
                format!("Failed to parse JSON: {}", e),
            ))
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.create_script(script_details) {
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
