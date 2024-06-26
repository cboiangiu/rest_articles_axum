pub mod features;
pub mod templates;
use self::features::{
    articles::article::get_article::map_get_article_endpoint,
    index::index_page::map_index_page_endpoint,
};
use super::api::{
    domain::articles::article::Article,
    persistence::{self, ArticleRepository},
};
use crate::framework::infrastructure::{
    persistence::mongo_repository::MongoEntityRepository, reverse_proxy::reverse_proxy_router,
};
use axum::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct WebState {
    pub article_repository: Arc<dyn ArticleRepository>,
}

pub async fn new_web_state() -> WebState {
    let collections = persistence::new_mongo_collections().await.unwrap();
    let article_repository = MongoEntityRepository::<Article>::new(collections.articles.clone());
    WebState {
        article_repository: Arc::new(article_repository.clone()),
    }
}

pub fn map_endpoints(state: WebState) -> axum::Router {
    Router::new()
        .merge(map_index_page_endpoint(state.clone()))
        .nest(
            "/article",
            Router::new().merge(map_get_article_endpoint(state.clone())),
        )
        .nest_service("/assets", tower_http::services::ServeDir::new("assets/web"))
        .nest_service(
            "/favicon.ico",
            tower_http::services::ServeFile::new("assets/web/favicon.ico"),
        )
        .nest(
            "/x/1",
            reverse_proxy_router("1".to_string(), "127.0.0.1:3001".to_string()),
        )
}
