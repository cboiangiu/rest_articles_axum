pub mod article_controller;
use crate::dtos::{
    article_dto::{CreateArticleDTO, ReadArticleDTO, UpdateArticleDTO},
    ErrorDTO, PaginatedReadArticleDTO,
};
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
struct PaginationParams {
    #[param(default = 1, minimum = 1)]
    page_number: usize,
    #[param(default = 10, minimum = 10)]
    page_size: usize,
}

impl IntoResponse for ErrorDTO {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        article_controller::insert_article,
        article_controller::get_article_by_id,
        article_controller::get_articles,
        article_controller::update_article,
        article_controller::delete_article,
    ),
    components(schemas(
        CreateArticleDTO,
        ReadArticleDTO,
        PaginatedReadArticleDTO,
        UpdateArticleDTO,
        ErrorDTO
    ))
)]
pub struct ApiDoc;
