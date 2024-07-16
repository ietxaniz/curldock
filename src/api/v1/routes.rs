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
        ("/v1/data", &Method::GET) => handlers::load_data_file(path).await,
        ("/v1/data", &Method::POST) => handlers::store_data_file(body).await,
        ("/v1/data", &Method::PUT) => handlers::update_data_file(body).await,
        ("/v1/data", &Method::DELETE) => handlers::delete_data_file(path).await,
        ("/v1/data/rename", &Method::POST) => handlers::rename_file(body).await,
        ("/v1/execute", &Method::POST) => handlers::execute_script(path).await,
        ("/v1/folder", &Method::POST) => handlers::create_folder(body).await,
        ("/v1/list", &Method::GET) => handlers::list_files_recursive().await,
        ("/v1/script", &Method::GET) => handlers::get_script_details(path).await,
        ("/v1/script", &Method::POST) => handlers::create_script(body).await,
        ("/v1/script", &Method::PUT) => handlers::update_script(body).await,
        ("/v1/script/rename", &Method::POST) => handlers::rename_script(body).await,

        _ => not_found::not_found().await,
    }
}
