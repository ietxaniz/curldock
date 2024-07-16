use crate::api::common::models::Response;
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameFileRequest {
    pub old_path: String,
    pub new_path: String,
}

pub async fn rename_file(body: web::Json<RenameFileRequest>) -> Result<HttpResponse, ApiError> {
    let script_manager = script_manager::get_script_manager();

    script_manager
        .rename_file(&body.old_path, &body.new_path)
        .map(|renamed_file| HttpResponse::Ok().json(Response::success(renamed_file)))
        .map_err(|e| ApiError::from_script_manager_error("rename_file", e))
}
