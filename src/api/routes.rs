use super::common::handlers::not_found;
use super::v1;
use actix_web::{http::Method, web, HttpRequest, HttpResponse};

pub async fn handle_request(
    path: &str,
    method: &Method,
    req: HttpRequest,
    body: web::Bytes,
) -> HttpResponse {
    let api_path = path.trim_start_matches("/api");

    match api_path {
        path if path.starts_with("/v1") => v1::handle_request(path, method, &req, body).await,
        _ => not_found::not_found().await,
    }
}
