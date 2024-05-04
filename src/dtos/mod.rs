pub mod article_dto;
use self::article_dto::ReadArticleDTO;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[aliases(PaginatedReadArticleDTO = PaginatedDTO<ReadArticleDTO>)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedDTO<T> {
    pub items: Vec<T>,
    pub page_number: usize,
    pub page_size: usize,
    pub total_items: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorDTO {
    pub message: String,
}
