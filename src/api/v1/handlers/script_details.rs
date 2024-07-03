use actix_web::{web, HttpResponse};
use crate::script_manager;

pub async fn get_script_details(path_info: web::Path<(String, String)>) -> HttpResponse {
    let (path, name) = path_info.into_inner();
    let script_manager = script_manager::get_script_manager();
    
    match script_manager.get_script_details(&path, &name) {
        Ok(script_details) => HttpResponse::Ok().json(script_details),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => HttpResponse::NotFound().body("Script not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get script details"),
    }
}