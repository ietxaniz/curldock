use crate::api::common::models::{Response, PathQuery};
use crate::api::common::ApiError;
use crate::script_manager;
use actix_web::{HttpResponse, ResponseError, web};

pub async fn load_data_file(query: Result<web::Query<PathQuery>, HttpResponse>) -> HttpResponse {
    match query {
        Ok(query) => {
            let script_manager = script_manager::get_script_manager();
            match script_manager.load_data_file(&query.path) {
                Ok(execution_result) => HttpResponse::Ok().json(Response::success(execution_result)),
                Err(e) => ApiError::from_script_manager_error("load_data_file", e).error_response(),
            }
        }
        Err(e) => e,
    }
}
