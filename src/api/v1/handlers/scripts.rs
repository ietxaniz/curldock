use actix_web::HttpResponse;
use crate::script_manager;

pub async fn list_scripts() -> HttpResponse {
    let script_manager = script_manager::get_script_manager();
    match script_manager.list_scripts() {
        Ok(scripts) => HttpResponse::Ok().json(scripts),
        Err(_) => HttpResponse::InternalServerError().body("Failed to list scripts"),
    }
}