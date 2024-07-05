use actix_web::{web, HttpRequest, HttpResponse, Error};
use awc::{Client, ws::Frame};
use futures_util::{StreamExt, future::ready};
use actix_web::web::Bytes;
use crate::config;

pub async fn handle_dev_proxy(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, Error> {
    let client = Client::default();
    let path = req.uri().to_string();

    let config = config::get_config();
    let forward_url = format!("{}{}", config.dev_server_url(), path);

    if req.headers().contains_key("upgrade") && req.headers().get("upgrade").unwrap() == "websocket" {
        println!("Handling WebSocket upgrade request for URL: {}", forward_url);

        let (response, connection) = client.ws(forward_url).connect().await.map_err(|e| {
            println!("WebSocket connection error: {:?}", e);
            actix_web::error::ErrorInternalServerError(format!("WebSocket connection error: {:?}", e))
        })?;

        let mut res = HttpResponse::build(response.status());
        for (header_name, header_value) in response.headers() {
            res.insert_header((header_name.clone(), header_value.clone()));
        }

        let stream = connection
            .take_while(|item| ready(item.is_ok()))
            .map(|item| item.unwrap())
            .map(|frame| {
                match frame {
                    Frame::Text(bytes) => Ok::<Bytes, Error>(Bytes::from(bytes)),
                    Frame::Binary(bytes) => Ok::<Bytes, Error>(Bytes::from(bytes)),
                    _ => Ok::<Bytes, Error>(Bytes::new()),
                }
            });

        return Ok(res.streaming(stream));
    } else {
        let mut forward_request = client.request_from(forward_url, req.head());

        // Forward headers from the original request
        for (header_name, header_value) in req.headers().iter() {
            forward_request = forward_request.insert_header((header_name.clone(), header_value.clone()));
        }

        match forward_request.send_body(body).await {
            Ok(mut res) => {
                let mut client_resp = HttpResponse::build(res.status());
                for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
                    client_resp.insert_header((header_name.clone(), header_value.clone()));
                }
                let body = res.body().await.unwrap_or_default();
                Ok(client_resp.body(body))
            },
            Err(err) => {
                println!("Error forwarding request to dev server: {:?}", err);
                Ok(HttpResponse::InternalServerError().body("Error forwarding request to dev server"))
            },
        }
    }
}
