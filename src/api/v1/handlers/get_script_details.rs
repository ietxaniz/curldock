use crate::api::common::models::{Response, PathQuery};
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError, web};

pub async fn get_script_details(query: Result<web::Query<PathQuery>, HttpResponse>) -> HttpResponse {
    match query {
        Ok(query) => {
            let script_manager = script_manager::get_script_manager();
            match script_manager.get_script_details(&query.path) {
                Ok(script_details) => HttpResponse::Ok().json(Response::success(script_details)),
                Err(e) => ApiError::from_script_manager_error("get_script_details", e).error_response(),
            }
        }
        Err(e) => e,
    }
}
