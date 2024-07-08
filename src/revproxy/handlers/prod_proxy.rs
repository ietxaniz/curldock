use actix_web::{HttpRequest, HttpResponse, Result};
use actix_files::NamedFile;
use std::path::PathBuf;
use mime_guess::from_path;

pub async fn handle_prod_proxy(req: HttpRequest) -> Result<HttpResponse> {
    // Get the full path from the request
    let path = req.uri().path().trim_start_matches('/');
    
    println!("Requested path: {}", path);

    let file_path = if path.is_empty() {
        PathBuf::from("front/dist/index.html")
    } else {
        PathBuf::from("front/dist").join(path)
    };

    println!("Looking for file: {:?}", file_path);

    if file_path.is_file() {
        println!("File found, serving: {:?}", file_path);
        let mime_type = from_path(&file_path).first_or_octet_stream();
        let file = NamedFile::open(file_path)?;
        Ok(file.set_content_type(mime_type).into_response(&req))
    } else {
        println!("File not found, falling back to index.html");
        let index_path = PathBuf::from("front/dist/index.html");
        if index_path.is_file() {
            println!("Serving index.html");
            let file = NamedFile::open(index_path)?;
            Ok(file.set_content_type(mime::TEXT_HTML).into_response(&req))
        } else {
            println!("index.html not found, returning 404");
            Ok(HttpResponse::NotFound().body("404 Not Found"))
        }
    }
}