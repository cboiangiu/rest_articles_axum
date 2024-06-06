use crate::{
    framework::core::errors::ApiError,
    modules::api::{persistence::ArticleRepository, ApiState},
};
use axum::{extract::State, http::StatusCode, routing::delete, Json, Router};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

pub fn map_delete_articles_v1_endpoint(state: ApiState) -> Router {
    Router::new()
        .route("/", delete(delete_articles_v1_endpoint))
        .with_state(state)
}

#[utoipa::path(
    delete,
    path = "/api/v1/article",   
    tag = "article",
    request_body = DeleteArticlesCommand,
    responses(
        (status = 204),
        (status = 400, body = ErrorResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn delete_articles_v1_endpoint(
    State(state): State<ApiState>,
    request: Json<DeleteArticlesCommand>,
) -> Result<(StatusCode, ()), ApiError> {
    handle(state.article_repository, request.0).await
}

pub async fn handle(
    repository: Arc<dyn ArticleRepository>,
    command: DeleteArticlesCommand,
) -> Result<(StatusCode, ()), ApiError> {
    delete_articles_validator(&command, repository.clone()).await?;

    for article in command.articles {
        repository.base().delete(article.clone()).await?;
    }

    Ok((StatusCode::NO_CONTENT, ()))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeleteArticlesCommand {
    pub articles: Vec<String>,
}

async fn delete_articles_validator(
    command: &DeleteArticlesCommand,
    repository: Arc<dyn ArticleRepository>,
) -> Result<(), ApiError> {
    if command.articles.is_empty() {
        return Err(ApiError::ArgumentNullError("articles".to_string()));
    }
    for article in command.articles.iter() {
        delete_article_validator(article.clone(), repository.clone()).await?;
    }
    Ok(())
}

async fn delete_article_validator(
    id: String,
    repository: Arc<dyn ArticleRepository>,
) -> Result<(), ApiError> {
    if ObjectId::parse_str(id.clone()).is_err() {
        return Err(ApiError::InvalidObjectIdError);
    }
    let item = repository.base().get_by_id(id).await?;
    match item {
        None => Err(ApiError::NotFoundError),
        _ => Ok(()),
    }
}
