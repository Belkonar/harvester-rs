#![allow(unused)] // This is a common file

use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Global error type
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Code: {}; {}", self.code, self.message)
    }
}

// This is to fix the warning
impl AppError {
    pub fn status(code: StatusCode) -> AppError {
        AppError {
            code,
            message: String::new(),
        }
    }

    pub fn new(code: StatusCode, message: String) -> AppError {
        AppError { code, message }
    }

    pub fn not_found() -> AppError {
        AppError {
            code: StatusCode::NOT_FOUND,
            message: "Not Found".to_string(),
        }
    }

    pub fn server_error(message: String) -> AppError {
        AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }

    pub fn error_from(obj: impl Display) -> AppError {
        AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: obj.to_string(),
        }
    }

    pub fn bad_request(message: String) -> AppError {
        AppError {
            code: StatusCode::BAD_REQUEST,
            message,
        }
    }

    /// Shorthand for server_error
    pub fn se(message: String) -> AppError {
        AppError::server_error(message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

/// Use this for most functions that return a result
pub type AppResult<T> = Result<T, AppError>;
pub type JsonResult<T> = AppResult<Json<T>>;

pub fn json_ok<T>(o: T) -> JsonResult<T> {
    Ok(Json(o))
}
