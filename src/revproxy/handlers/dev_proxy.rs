use actix_web::{web, HttpRequest, HttpResponse, Responder};
use awc::Client;
use crate::config::Config;

pub async fn handle_dev_proxy(req: HttpRequest, body: web::Bytes, config: web::Data<Config>) -> impl Responder {
    let client = Client::default();
    let path = req.uri().to_string();
    let forward_url = format!("{}{}", config.dev_server_url(), path);

    match client.request_from(forward_url, req.head()).send_body(body).await {
        Ok(mut res) => {
            let mut client_resp = HttpResponse::build(res.status());
            for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
                client_resp.insert_header((header_name.clone(), header_value.clone()));
            }
            let body = res.body().await.unwrap_or_default();
            client_resp.body(body)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error forwarding request to dev server"),
    }
}