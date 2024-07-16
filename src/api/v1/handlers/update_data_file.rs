use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct UpdateDataFileRequest {
    pub path: String,
    pub content: HashMap<String, String>,
}

pub async fn update_data_file(body: web::Json<UpdateDataFileRequest>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    match script_manager.update_data_file(&body.path, &body.content) {
        Ok(updated_file) => HttpResponse::Ok().json(Response::success(updated_file)),
        Err(e) => ApiError::from_script_manager_error("update_data_file", e).error_response(),
    }
}
