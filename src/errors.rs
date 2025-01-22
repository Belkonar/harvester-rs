#![allow(dead_code)] // This is a common file

use std::fmt::Display;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Global error type
/// Use in basically all scenarios where an error is needed.
#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Code: {}", self.code.as_u16())?;
        writeln!(f, "{}", self.message)
    }
}

impl AppError {
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

    pub fn bad_request(message: String) -> AppError {
        AppError {
            code: StatusCode::BAD_REQUEST,
            message,
        }
    }

    /// implementing this here instead of a trait fixes conflict issues
    pub fn from<T: Display>(obj: T) -> AppError {
        AppError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: obj.to_string(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let err = AppError {
            code: StatusCode::OK,
            message: "ok".to_string(),
        };

        assert_eq!(err.to_string(), "Code: 200\nok\n");
    }

    #[test]
    fn test_from() {
        let err = sqlx::Error::PoolClosed {};
        let err2: AppError = AppError::from(err);

        assert_eq!(
            err2.message,
            "attempted to acquire a connection on a closed pool"
        );
        assert_eq!(err2.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_json() {
        let resp: JsonResult<String> = json_ok("hi".to_string());
        assert_eq!(resp.unwrap().to_string(), "hi");
    }
}
