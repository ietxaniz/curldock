use actix_web::{web, HttpResponse};
use crate::script_manager;
use crate::api::common::models::Response;

pub async fn list_scripts(path: web::Path<String>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();
    let path_str = path.into_inner();
    let path = if path_str.is_empty() { None } else { Some(path_str.as_str()) };

    match script_manager.list_scripts(path) {
        Ok(script_list) => HttpResponse::Ok().json(Response::success(script_list)),
        Err(e) => {
            let response = Response::<()>::error(
                "IoError".to_string(),
                format!("Failed to list scripts: {}", e),
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

