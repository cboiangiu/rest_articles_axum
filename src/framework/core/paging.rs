use crate::modules::api::features::articles::article::get_article_list_v1::GetArticleListResponse;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(PaginatedGetArticleListResponse = PaginatedDTO<GetArticleListResponse>)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedDTO<T> {
    pub items: Vec<T>,
    pub page_number: usize,
    pub page_size: usize,
    pub total_items: u64,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    #[param(default = 1, minimum = 1)]
    pub page_number: usize,
    #[param(default = 10, minimum = 10)]
    pub page_size: usize,
}
