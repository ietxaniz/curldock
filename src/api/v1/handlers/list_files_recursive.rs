use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError};

pub async fn list_files_recursive() -> HttpResponse {
    let script_manager = script_manager::get_script_manager();

    match script_manager.list_files_recursive() {
        Ok(file_list) => HttpResponse::Ok().json(Response::success(file_list)),
        Err(e) => ApiError::from_script_manager_error("list_files_recursive", e).error_response(),
    }
}
