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
        write!(f, "Code: {}; {};", self.code.as_u16(), self.message)
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

    /// TODO: Think about not having this. I'm unsure of its value
    pub fn from_code<T: Display>(code: StatusCode) -> impl Fn(T) -> AppError {
        move |e| AppError {
            code,
            message: e.to_string(),
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

pub fn json_ok<T>(obj: T) -> JsonResult<T> {
    Ok(Json(obj))
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

        assert_eq!(err.to_string(), "Code: 200; ok;");
    }

    /// Test the from method. It should make an error from any object that implements `Display`
    #[test]
    fn test_from() {
        let err = sqlx::Error::PoolClosed {};
        let err2: AppError = AppError::from(&err);

        assert_eq!(err2.message, err.to_string());
        assert_eq!(err2.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// Test that the types are all correct for `json_ok`.
    #[test]
    fn test_json() {
        let resp: JsonResult<String> = json_ok("hi".to_string());
        assert_eq!(resp.unwrap().to_string(), "hi");
    }
}
