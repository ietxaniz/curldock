use super::handlers;
use crate::api::common::handlers::not_found;
use crate::script_manager::{self, models::ScriptDetailsCreate};
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

pub async fn handle_request(
    path: &str,
    method: &Method,
    _req: &HttpRequest,
    body: web::Bytes,
) -> HttpResponse {
    let script_manager = script_manager::get_script_manager().clone();

    match (path, method) {
        (p, &Method::GET) if p.starts_with("/v1/scripts") => {
            let script_path = p
                .trim_start_matches("/v1/scripts")
                .trim_start_matches('/')
                .to_string();
            handlers::list_scripts::list_scripts(web::Path::from(script_path)).await
        }
        ("/v1/scrrecursive", &Method::GET) => {
            handlers::list_scripts_recursive::list_scripts_recursive().await
        }
        (p, &Method::GET) if p.starts_with("/v1/script-details") => {
            let remaining_path = p
                .trim_start_matches("/v1/script-details")
                .trim_start_matches('/');
            let parts: Vec<&str> = remaining_path.splitn(2, '/').collect();
            match parts.len() {
                1 => {
                    handlers::script_details::get_script_details(
                        web::Data::new(script_manager.clone()),
                        web::Path::from(("".to_string(), parts[0].to_string())),
                    )
                    .await
                }
                2 => {
                    handlers::script_details::get_script_details(
                        web::Data::new(script_manager.clone()),
                        web::Path::from((parts[0].to_string(), parts[1].to_string())),
                    )
                    .await
                }
                _ => not_found::not_found().await,
            }
        }
        (p, &Method::POST) if p.starts_with("/v1/execute") => {
            let remaining_path = p.trim_start_matches("/v1/execute").trim_start_matches('/');
            let parts: Vec<&str> = remaining_path.splitn(2, '/').collect();
            match parts.len() {
                1 => {
                    handlers::execute_script::execute_script(web::Path::from((
                        "".to_string(),
                        parts[0].to_string(),
                    )))
                    .await
                }
                2 => {
                    handlers::execute_script::execute_script(web::Path::from((
                        parts[0].to_string(),
                        parts[1].to_string(),
                    )))
                    .await
                }
                _ => not_found::not_found().await,
            }
        }
        ("/v1/script", &Method::POST) => {
            let script_details: ScriptDetailsCreate = match serde_json::from_slice(&body) {
                Ok(data) => data,
                Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
            };
            handlers::create_script::create_script(web::Json(script_details)).await
        }
        ("/v1/script", &Method::PUT) => {
            let script_details: ScriptDetailsCreate = match serde_json::from_slice(&body) {
                Ok(data) => data,
                Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
            };
            handlers::update_script::update_script(web::Json(script_details)).await
        }
        ("/v1/rename-script", &Method::POST) => {
            let rename_request = match serde_json::from_slice(&body) {
                Ok(data) => data,
                Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
            };
            handlers::rename_script::rename_script(web::Json(rename_request)).await
        }
        ("/v1/create-folder", &Method::POST) => {
            let folder_request = match serde_json::from_slice(&body) {
                Ok(data) => data,
                Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
            };
            handlers::create_folder::create_folder(web::Json(folder_request)).await
        }
        // Add other v1 routes here as you develop them
        _ => not_found::not_found().await,
    }
}
