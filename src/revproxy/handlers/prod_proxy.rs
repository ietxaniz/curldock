use actix_web::{HttpRequest, HttpResponse, Responder};
use std::path::PathBuf;

pub async fn handle_prod_proxy(req: HttpRequest) -> impl Responder {
    // This is a placeholder. Implement your static file serving or SPA serving logic here.
    let path = req.uri().path();
    let file_path = PathBuf::from("static").join(path.trim_start_matches('/'));

    if file_path.is_file() {
        // Serve the static file
        HttpResponse::Ok().body(format!("Serving static file: {:?}", file_path))
    } else {
        // Serve the SPA (usually index.html)
        HttpResponse::Ok().body("Serving SPA")
    }
}