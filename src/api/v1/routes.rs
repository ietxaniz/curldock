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
        // Add other v1 routes here as you develop them
        _ => not_found::not_found().await,
    }
}