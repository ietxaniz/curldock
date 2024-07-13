use crate::test_endpoints;
use actix_web::{web, HttpRequest, HttpResponse, http::StatusCode};

pub async fn handle_test(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let req_path = req.uri().path();
    if let Some(path) = req_path.strip_prefix("/test") {
        let method = req.method().clone();
        test_endpoints::handle_request(path, &method, &req, body).await
    } else {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Reverse proxy error: path should have started with /test")
    }
}
