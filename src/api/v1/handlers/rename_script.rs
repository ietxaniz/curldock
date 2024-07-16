use crate::api::common::models::Response;
use crate::api::common::{ApiError,ErrorKind};
use crate::script_manager;
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RenameScriptRequest {
    #[serde(rename = "oldName")]
    pub old_name: String,
    #[serde(rename = "newName")]
    pub new_name: String,
    #[serde(rename = "oldPath")]
    pub old_path: String,
    #[serde(rename = "newPath")]
    pub new_path: String,
}

pub async fn rename_script(body: web::Bytes) -> Result<HttpResponse, ApiError> {
    let rename_request: RenameScriptRequest = serde_json::from_slice(&body).map_err(|e| {
        ApiError::new(
            ErrorKind::InvalidInput,
            "rename_script",
            format!("Failed to parse JSON: {}", e),
        )
    })?;

    let script_manager = script_manager::get_script_manager();

    script_manager
        .rename_script(
            &rename_request.old_path,
            &rename_request.old_name,
            &rename_request.new_path,
            &rename_request.new_name,
        )
        .map(|renamed_script| HttpResponse::Ok().json(Response::success(renamed_script)))
        .map_err(|e| ApiError::from_script_manager_error("rename_script", e))
}
