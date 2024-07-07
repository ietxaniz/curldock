use actix_web::HttpResponse;
use crate::script_manager::{self, models::ScriptError};
use crate::api::common::models::Response;

pub async fn list_scripts_recursive() -> HttpResponse {
  let script_manager = script_manager::get_script_manager();

  match script_manager.list_scripts_recursive() {
      Ok(script_list) => HttpResponse::Ok().json(Response::success(script_list)),
      Err(err) => {
          let (mut status, response) = match err {
              ScriptError::IoError(e) => (
                  HttpResponse::InternalServerError(),
                  Response::<()>::error(
                      "IoError".to_string(),
                      format!("An IO error occurred: {}", e),
                  ),
              ),
              ScriptError::InvalidPath(e) => (
                  HttpResponse::BadRequest(),
                  Response::<()>::error(
                      "InvalidPath".to_string(),
                      format!("Invalid path: {}", e),
                  ),
              ),
              _ => (
                  HttpResponse::InternalServerError(),
                  Response::<()>::error(
                      "UnexpectedError".to_string(),
                      "An unexpected error occurred".to_string(),
                  ),
              ),
          };
          status.json(response)
      }
  }
}
