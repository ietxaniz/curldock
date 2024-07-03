use actix_web::{HttpRequest, HttpResponse, http::Method, web};
use super::handlers;
use crate::api::common::handlers::not_found;

pub async fn handle_request(
    path: &str,
    method: &Method,
    _req: &HttpRequest,
    _body: web::Bytes
) -> HttpResponse {
    match (path, method) {
        ("/v1/hello", &Method::GET) => handlers::hello::hello().await,
        (p, &Method::GET) if p.starts_with("/v1/scripts") => {
            let script_path = p.trim_start_matches("/v1/scripts").trim_start_matches('/').to_string();
            handlers::scripts::list_scripts(web::Path::from(script_path)).await
        },
        // Add other v1 routes here as you develop them
        _ => not_found::not_found().await,
    }
}