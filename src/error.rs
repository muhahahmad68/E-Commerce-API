use axum::{
    response::{IntoResponse, Response},
    http::{StatusCode},
};

#[derive(Clone)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    JsonError(String),
    Unauthorized(String),
    FORBIDDEN(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::ValidationError(err)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JsonError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DatabaseError(e) => {
                eprintln!("Database error {:}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::ValidationError(msg) => {
                eprintln!("Validation error {:?}", msg);
                (StatusCode::BAD_REQUEST, msg.clone()).into_response()
            }
            AppError::JsonError(msg) => {
                eprintln!("Json error {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()).into_response()
            }
            AppError::Unauthorized(msg) => {
                eprintln!("Token validation error {}", msg);
                (StatusCode::UNAUTHORIZED, msg.clone()).into_response()

            }
            AppError::FORBIDDEN(msg) => {
                (StatusCode::UNAUTHORIZED, msg.clone()).into_response()
            }
        }
    }
}