use diesel::result::Error as DieselError;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub error: Option<serde_json::Value>, // Dynamic details
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<DieselError> for ApiError {
    fn from(err: DieselError) -> Self {
        ApiError {
            message: "Database error".to_string(),
            error: Some(serde_json::json!({ "details": err.to_string() })),
        }
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(err: bcrypt::BcryptError) -> Self {
        ApiError {
            message: "Password hashing error".to_string(),
            error: Some(serde_json::json!({ "details": err.to_string() })),
        }
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError {
            message: err,
            error: None,
        }
    }
}
