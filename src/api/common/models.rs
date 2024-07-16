use serde::{Serialize, Deserialize};
use actix_web::{web, HttpResponse};
use crate::api::common::{ApiError, ErrorKind};
use serde::de::DeserializeOwned;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(rename = "errorDetails")]
    pub error_details: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub error: Option<ErrorResponse>,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error_type: String, error_details: String) -> Self {
        Response {
            data: None,
            error: Some(ErrorResponse {
                error_type,
                error_details,
            }),
        }
    }
}

#[derive(Deserialize)]
pub struct PathQuery {
    pub path: String,
}

pub fn decode_query<T: DeserializeOwned>(path: &str) -> Result<web::Query<T>, HttpResponse> {
  let query_str = path.split('?').nth(1).unwrap_or("");
  web::Query::<T>::from_query(query_str).map_err(|_| {
      HttpResponse::BadRequest().json(ApiError::new(
          ErrorKind::InvalidInput,
          "decode_query",
          "Invalid query parameters".to_string(),
      ))
  })
}