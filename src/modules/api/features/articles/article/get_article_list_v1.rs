use crate::framework::core::domain::EntityWithId;
use crate::{
    domain::articles::article::Article,
    framework::core::{
        errors::ApiError,
        paging::{PaginatedDTO, PaginationParams},
    },
    modules::api::{persistence::ArticleRepository, ApiModuleState},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

pub fn map_get_article_list_v1_endpoint(state: ApiModuleState) -> Router {
    Router::new()
        .route("/", get(get_article_list_v1_endpoint))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/api/v1/article",   
    tag = "article",
    params(PaginationParams),
    responses(
        (status = 200, body = PaginatedGetArticleListResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn get_article_list_v1_endpoint(
    State(state): State<ApiModuleState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<(StatusCode, Json<PaginatedDTO<GetArticleListResponse>>), ApiError> {
    get_article_list_handler(
        state.article_repository,
        GetArticleListQuery {
            page_number: pagination.page_number,
            page_size: pagination.page_size,
        },
    )
    .await
}

pub async fn get_article_list_handler(
    repository: Arc<dyn ArticleRepository>,
    query: GetArticleListQuery,
) -> Result<(StatusCode, Json<PaginatedDTO<GetArticleListResponse>>), ApiError> {
    let items = repository
        .base()
        .get_all_paginated(query.page_number, query.page_size)
        .await?;
    let response = PaginatedDTO {
        items: items
            .data
            .iter()
            .map(|article| article.clone().into())
            .collect(),
        total_items: items.total,
        page_number: query.page_number,
        page_size: query.page_size,
    };
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleListQuery {
    pub page_number: usize,
    pub page_size: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, o2o)]
#[from_owned(Article)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleListResponse {
    #[ghost(@.get_id().to_hex())]
    pub id: String,
    pub title: String,
    pub content: String,
    pub published_date: DateTime<Utc>,
}
