use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Error, ToSchema)]
pub enum ApiError {
    #[error("ArgumentNullError: {0}")]
    ArgumentNullError(String),
    #[error("InvalidObjectIdError")]
    InvalidObjectIdError,
    #[error("TextTooLongError: {0}")]
    TextTooLongError(String),
    #[error("NotFoundError")]
    NotFoundError,
    #[error("DuplicateEntityError")]
    DuplicateEntityError,
    #[error("InternalServerError")]
    InternalServerError,
}

impl Into<ErrorResponse> for ApiError {
    fn into(self) -> ErrorResponse {
        ErrorResponse {
            message: self.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ApiError::ArgumentNullError(_) => StatusCode::BAD_REQUEST,
            ApiError::InvalidObjectIdError => StatusCode::BAD_REQUEST,
            ApiError::TextTooLongError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFoundError => StatusCode::NOT_FOUND,
            ApiError::DuplicateEntityError => StatusCode::CONFLICT,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let erro: ErrorResponse = self.into();

        (status_code, erro).into_response()
    }
}
