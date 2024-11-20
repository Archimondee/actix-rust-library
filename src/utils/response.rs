use serde::Serialize;
use serde_json::Value;

use super::errors::ApiError;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub message: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

pub fn create_response<T: Serialize>(
    message: &str,
    status: u16,
    data: Option<T>,
    meta: Option<Value>,
    error: Option<ApiError>,
) -> ApiResponse<T> {
    ApiResponse {
        message: message.to_string(),
        status,
        data,
        meta,
        error,
    }
}
