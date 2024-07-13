use actix_web::{web, HttpResponse, http::StatusCode};
use crate::script_manager;
use crate::api::common::models::Response;

pub async fn list_scripts(path: web::Path<String>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();
    let path_str = path.into_inner();

    let script_path = if path_str == "/v1/scripts" {
        ""
    } else if let Some(stripped) = path_str.strip_prefix("/v1/scripts/") {
        stripped
    } else {
        return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Reverse proxy error: path should have started with /v1/scripts");
    };

    let path = if script_path.is_empty() { None } else { Some(script_path) };

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
