use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Global error type
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn status(code: StatusCode) -> AppError {
        AppError {
            code,
            message: "".to_string(),
        }
    }

    pub fn new(code: StatusCode, message: String) -> AppError {
        AppError { code, message }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}

/// Use this for most functions that return a result
pub type AppResult<T> = Result<T, AppError>;
