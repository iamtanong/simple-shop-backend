use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Default)]
pub enum AppError {
    #[default]
    #[display("Internal Server Error")]
    InternalServerError, // 500
    #[display("Bad Request: {}", _0)]
    BadRequest(String), // 400
    #[display("Unauthorized")]
    Unathorized, // 401
    #[display("Forbidden")]
    Forbidden, // 403
    #[display("Not Found")]
    NotFound, // 404
}

impl std::error::Error for AppError {}

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unathorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
