use actix_web::{web, HttpRequest, HttpResponse};
use crate::config::Config;
use crate::api;

pub async fn handle_api(req: HttpRequest, body: web::Bytes, config: web::Data<Config>) -> HttpResponse {
    let path = req.uri().path().trim_start_matches("/api");
    let method = req.method().clone();

    api::handle_request(path, &method, req.clone(), body, config).await
}