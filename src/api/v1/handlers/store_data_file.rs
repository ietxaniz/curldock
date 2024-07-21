use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct StoreDataFileDetails {
    pub path: String,
    pub content: HashMap<String, String>,
}

pub async fn store_data_file(body: web::Bytes) -> HttpResponse {
    let body_str = String::from_utf8_lossy(&body);
    println!("Received body: {}", body_str);

    let store_data_file_details: StoreDataFileDetails = match serde_json::from_slice(&body) {
        Ok(details) => details,
        Err(e) => {
            return ApiError::new(
                "store_data_file",
                format!("Failed to parse JSON: {}", e),
            )
            .error_response()
        }
    };

    let script_manager = script_manager::get_script_manager();

    match script_manager.store_data_file(
        &store_data_file_details.path,
        &store_data_file_details.content,
    ) {
        Ok(created_data_file) => HttpResponse::Created().json(Response::success(created_data_file)),
        Err(e) => {
            ApiError::from_debug_error("store_data_file", e).error_response()
        }
    }
}
