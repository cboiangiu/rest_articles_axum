use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error, ToSchema)]
pub enum ArticleError {
    #[error("ArgumentNullError:{0}")]
    ArgumentNullError(String),
    #[error("TextTooLongError:{0}")]
    TextTooLongError(String),
    #[error("NotFoundError")]
    NotFoundError,
}
