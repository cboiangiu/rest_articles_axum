use crate::{
    domain::articles::article::Article,
    framework::core::{domain::EntityWithId, errors::ApiError},
    modules::api::{persistence::ArticleRepository, ApiModuleState},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

pub fn map_get_article_v1_endpoint(state: ApiModuleState) -> Router {
    Router::new()
        .route("/:id", get(get_article_v1_endpoint))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/api/v1/article/{id}",   
    tag = "article",
    params(
        ("id" = String, Path,)
    ),
    responses(
        (status = 200, body = GetArticleResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn get_article_v1_endpoint(
    State(state): State<ApiModuleState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<GetArticleResponse>), ApiError> {
    get_article_handler(state.article_repository, GetArticleQuery { id }).await
}

pub async fn get_article_handler(
    repository: Arc<dyn ArticleRepository>,
    query: GetArticleQuery,
) -> Result<(StatusCode, Json<GetArticleResponse>), ApiError> {
    if ObjectId::parse_str(query.id.clone()).is_err() {
        return Err(ApiError::InvalidObjectIdError);
    }
    let item = repository.base().get_by_id(query.id).await?;
    match item {
        Some(item) => {
            return Ok((StatusCode::OK, Json(item.into())));
        }
        None => {
            return Err(ApiError::NotFoundError);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleQuery {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, o2o)]
#[from_owned(Article)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleResponse {
    #[ghost(@.get_id().to_hex())]
    pub id: String,
    pub title: String,
    pub content: String,
    pub published_date: DateTime<Utc>,
}
