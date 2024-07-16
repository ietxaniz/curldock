use actix_web::{web, HttpRequest, HttpResponse, cookie::Cookie};
use serde_json::{json, Value};

pub async fn handle_reversing_post(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let auth_cookie = req.cookie("auth").map(|c| c.value().to_string());
    let received_data: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let message = received_data.get("message").and_then(Value::as_str).unwrap_or_default();

    let reversed_auth = auth_cookie.map(|auth| auth.chars().rev().collect::<String>());
    let reversed_message = message.chars().rev().collect::<String>();

    let response_json = json!({
        "message": reversed_message,
        "received": received_data
    });

    let mut response = HttpResponse::Created().json(response_json);
    if let Some(rev_auth) = reversed_auth {
        response.add_cookie(&Cookie::new("auth", rev_auth)).unwrap();
    }

    response
}