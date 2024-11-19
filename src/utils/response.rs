use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub message: String,
    pub status: u16,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}

pub fn createResponse<T: Serialize>(message: &str, status: u16, data: T, meta: Option<Value>) -> ApiResponse<T> {
    ApiResponse {
        message: message.to_string(),
        status,
        data,
        meta,
    }
}