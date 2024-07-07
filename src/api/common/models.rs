use serde::{Serialize, Deserialize};

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
