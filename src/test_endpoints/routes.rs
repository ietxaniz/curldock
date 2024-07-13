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
        ("/get", &Method::GET) => {
            handlers::get_handler::handle_get().await
        }
        ("/post", &Method::POST) => {
            handlers::post_handler::handle_post(web::Bytes::from(body)).await
        }
        ("/put", &Method::PUT) => {
            handlers::put_handler::handle_put(web::Bytes::from(body)).await
        }
        ("/delete", &Method::DELETE) => {
            handlers::delete_handler::handle_delete().await
        }
        ("/error", &Method::GET) => {
            handlers::error_handler::handle_error().await
        }
        _ => not_found::not_found().await,
    }
}
