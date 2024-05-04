use crate::dtos::ErrorDTO;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error, ToSchema)]
pub enum ArticleError {
    #[error("ArgumentNullError: {0}")]
    ArgumentNullError(String),
    #[error("InvalidObjectIdError")]
    InvalidObjectIdError,
    #[error("TextTooLongError: {0}")]
    TextTooLongError(String),
    #[error("NotFoundError")]
    NotFoundError,
}

impl Into<ErrorDTO> for ArticleError {
    fn into(self) -> ErrorDTO {
        ErrorDTO {
            message: self.to_string(),
        }
    }
}
