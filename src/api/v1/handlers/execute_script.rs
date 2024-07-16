use crate::api::common::models::{PathQuery, Response};
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{web, HttpResponse, ResponseError};

pub async fn execute_script(query: Result<web::Query<PathQuery>, HttpResponse>) -> HttpResponse {
    match query {
        Ok(query) => {
            let script_manager = script_manager::get_script_manager();
            match script_manager.execute_script(&query.path) {
                Ok(execution_result) => {
                    HttpResponse::Ok().json(Response::success(execution_result))
                }
                Err(e) => ApiError::from_script_manager_error("execute_script", e).error_response(),
            }
        }
        Err(e) => e,
    }
}
