use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteFileQuery {
    pub path: String,
}

pub async fn delete_data_file(query: web::Query<DeleteFileQuery>) -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    match script_manager.delete_data_file(&query.path) {
        Ok(_) => HttpResponse::Ok().json(Response::success("File deleted successfully")),
        Err(e) => ApiError::from_script_manager_error("delete_data_file", e).error_response(),
    }
}
