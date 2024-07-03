use actix_web::{web, HttpResponse};
use crate::script_manager::{self, models::{ScriptDetailsCreate, ScriptError}};

pub async fn create_script(script_details: web::Json<ScriptDetailsCreate>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();
    
    match script_manager.create_script(script_details.into_inner()) {
        Ok(created_script) => HttpResponse::Created().json(created_script),
        Err(ScriptError::IoError(e)) => {
            HttpResponse::InternalServerError().body(format!("An IO error occurred: {}", e))
        },
        Err(ScriptError::CurlParseError(_)) => {
            HttpResponse::BadRequest().body("Failed to parse curl command in script")
        },
    }
}
