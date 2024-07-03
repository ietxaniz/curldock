use actix_web::{HttpRequest, HttpResponse, http::Method, web};
use crate::config::Config;
use super::handlers::hello;
use super::common::handlers::not_found;
use super::v1;

pub async fn handle_request(
    path: &str,
    method: &Method,
    req: HttpRequest,
    body: web::Bytes,
    _config: web::Data<Config>
) -> HttpResponse {
    // Remove the leading "/api" from the path
    let api_path = path.trim_start_matches("/api");

    match api_path {
        "/hello" if method == &Method::GET => hello::hello().await,
        path if path.starts_with("/v1") => v1::handle_request(path, method, &req, body).await,
        // Add other top-level API routes here if needed
        _ => not_found::not_found().await,
    }
}