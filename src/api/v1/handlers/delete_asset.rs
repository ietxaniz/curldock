use crate::api::common::models::{decode_query, PathQuery, Response};
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError};


pub async fn delete_asset(path:&str) -> HttpResponse {
  let query = decode_query::<PathQuery>(path);
  match query {
      Ok(query) => {
          let script_manager = script_manager::get_script_manager();
          match script_manager.delete_asset(&query.path) {
              Ok(_) => HttpResponse::Ok().json(Response::success("Asset deleted successfully")),
              Err(e) => ApiError::from_debug_error("delete_asset", e).error_response(),
          }
      }
      Err(e) => e,
  }
}