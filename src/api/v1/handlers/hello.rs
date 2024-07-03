use actix_web::HttpResponse;

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello from API v1!")
}