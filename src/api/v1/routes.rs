use super::handlers;
use crate::api::common::handlers::not_found;
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

pub async fn handle_request(
    path: &str,
    method: &Method,
    _req: &HttpRequest,
    body: web::Bytes,
) -> HttpResponse {
    match (path, method) {
        ("/v1/files", &Method::GET) => {
            handlers::list_files_recursive::list_files_recursive().await
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
        ("/v1/create-folder", &Method::POST) => {
            handlers::create_folder::create_folder(web::Bytes::from(body)).await
        }
        ("/v1/file", &Method::DELETE) => {
            handlers::delete_data_file::delete_data_file(web::Query::from_query(path.split('?').nth(1).unwrap_or("")).unwrap()).await
        }
        _ => not_found::not_found().await,
    }
}