use crate::api::common::models::{decode_query, PathQuery, Response};
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError};

pub async fn get_script_details(path:&str) -> HttpResponse {
  let query = decode_query::<PathQuery>(path);
    match query {
        Ok(query) => {
            let script_manager = script_manager::get_script_manager();
            match script_manager.get_script_details(&query.path) {
                Ok(script_details) => HttpResponse::Ok().json(Response::success(script_details)),
                Err(e) => ApiError::from_debug_error("get_script_details", e).error_response(),
            }
        }
        Err(e) => e,
    }
}
