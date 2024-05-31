use crate::modules::{
    api::{features::articles::article::get_article_v1, persistence::ArticleRepository},
    web::WebModuleState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn map_get_article_endpoint(state: WebModuleState) -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |State(state): State<WebModuleState>, Path(id): Path<String>| {
                    get_article_handler(state.article_repository, GetArticleQuery { id })
                },
            ),
        )
        .with_state(state)
}

async fn get_article_handler(
    repository: Arc<dyn ArticleRepository>,
    query: GetArticleQuery,
) -> impl IntoResponse {
    let api_query = get_article_v1::GetArticleQuery { id: query.id };
    let item = get_article_v1::get_article_handler(repository, api_query).await;
    match item {
        Ok(item) => {
            return item.1.into_response();
        }
        Err(_) => {
            return "not found".to_string().into_response();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetArticleQuery {
    pub id: String,
}
