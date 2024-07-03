use actix_web::{web, HttpResponse};
use crate::script_manager::{self, models::ScriptError};

pub async fn execute_script(path_info: web::Path<(String, String)>) -> HttpResponse {
    let (path, name) = path_info.into_inner();
    let script_manager = script_manager::get_script_manager();
    
    match script_manager.execute_script(&path, &name) {
        Ok(execution_result) => HttpResponse::Ok().json(execution_result),
        Err(ScriptError::IoError(e)) if e.kind() == std::io::ErrorKind::NotFound => {
            HttpResponse::NotFound().body("Script not found")
        },
        Err(ScriptError::CurlParseError(_)) => {
            HttpResponse::BadRequest().body("Failed to parse curl command in script")
        },
        Err(ScriptError::IoError(_)) => {
            HttpResponse::InternalServerError().body("An IO error occurred while executing the script")
        },
    }
}
