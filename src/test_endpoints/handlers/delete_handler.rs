use actix_web::HttpResponse;

pub async fn handle_delete() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
