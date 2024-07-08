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
            handlers::script_details::get_script_details(p.to_string()).await
        }
        (p, &Method::POST) if p.starts_with("/v1/execute") => {
            handlers::execute_script::execute_script(p.to_string()).await
        }
        ("/v1/script", &Method::POST) => {
            handlers::create_script::create_script(web::Bytes::from(body)).await
        }
        ("/v1/script", &Method::PUT) => {
            handlers::update_script::update_script(web::Bytes::from(body)).await
        }
        ("/v1/rename-script", &Method::POST) => {
            handlers::rename_script::rename_script(web::Bytes::from(body)).await
        }
        ("/v1/create-folder", &Method::POST) => {
            handlers::create_folder::create_folder(web::Bytes::from(body)).await
        }
        _ => not_found::not_found().await,
    }
}
