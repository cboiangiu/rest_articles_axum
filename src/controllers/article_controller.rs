use super::PaginationParams;
use crate::{
    dtos::article_dto::{CreateArticleDTO, ReadArticleDTO, UpdateArticleDTO},
    errors::ArticleError,
    services::article_service::ArticleService,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ArticleControllerState {
    pub article_service: Arc<dyn ArticleService>,
}

#[utoipa::path(
    post,
    path = "/v1/articles",
    request_body = CreateArticleDTO,
    responses(
        (status = 201, body = ReadArticleDTO),
        (status = 400, body = ArticleError)
    )
)]
async fn insert_article(
    State(state): State<ArticleControllerState>,
    article: Json<CreateArticleDTO>,
) -> Result<(StatusCode, Json<ReadArticleDTO>), ArticleError> {
    let result = state.article_service.insert_article(article.0).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    get,
    path = "/v1/articles/{id}",
    responses(
        (status = 200, body = ReadArticleDTO),
        (status = 404, body = ArticleError)
    ),
    params(
        ("id" = i32, Path,)
    )
)]
async fn get_article_by_id(
    State(state): State<ArticleControllerState>,
    Path(id): Path<i32>,
) -> Result<Json<ReadArticleDTO>, ArticleError> {
    let result = state.article_service.get_article_by_id(id).await?;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/v1/articles",
    responses(
        (status = 200, body = Vec<ReadArticleDTO>)
    ),
    params(PaginationParams),
)]
async fn get_articles(
    State(state): State<ArticleControllerState>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<Vec<ReadArticleDTO>>, ArticleError> {
    let result = state
        .article_service
        .get_articles_paginated(pagination.page_number, pagination.page_size)
        .await?;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/v1/articles/{id}",
    request_body = UpdateArticleDTO,
    responses(
        (status = 200, body = ReadArticleDTO),
        (status = 400, body = ArticleError),
        (status = 404, body = ArticleError)
    ),
    params(
        ("id" = i32, Path,)
    )
)]
async fn update_article(
    State(state): State<ArticleControllerState>,
    Path(id): Path<i32>,
    article: Json<UpdateArticleDTO>,
) -> Result<Json<ReadArticleDTO>, ArticleError> {
    let result = state.article_service.update_article(id, article.0).await?;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/v1/articles/{id}",
    responses(
        (status = 204),
        (status = 404, body = ArticleError)
    ),
    params(
        ("id" = i32, Path,)
    )
)]
async fn delete_article(
    State(state): State<ArticleControllerState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ArticleError> {
    state.article_service.delete_article(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn route(state: ArticleControllerState) -> Router {
    Router::new()
        .nest(
            "/articles",
            Router::new()
                .route("/", post(insert_article))
                .route("/:id", get(get_article_by_id))
                .route("/", get(get_articles))
                .route("/:id", put(update_article))
                .route("/:id", delete(delete_article)),
        )
        .with_state(state)
}

impl IntoResponse for ArticleError {
    fn into_response(self) -> Response {
        let (status_code, body) = match self {
            ArticleError::ArgumentNullError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ArticleError::TextTooLongError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ArticleError::NotFoundError => (StatusCode::NOT_FOUND, self.to_string()),
        };

        (status_code, body).into_response()
    }
}
