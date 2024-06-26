pub mod domain;
pub mod features;
pub mod persistence;
use self::features::articles::article::{
    create_update_articles_v1::map_create_update_articles_v1_endpoint,
    delete_articles_v1::map_delete_articles_v1_endpoint,
    get_article_list_v1::map_get_article_list_v1_endpoint,
    get_article_v1::map_get_article_v1_endpoint,
};
use crate::framework::infrastructure::persistence::mongo_repository::MongoEntityRepository;
use axum::Router;
use domain::articles::article::Article;
use persistence::ArticleRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub article_repository: Arc<dyn ArticleRepository>,
}

pub async fn new_api_state() -> ApiState {
    let collections = persistence::new_mongo_collections().await.unwrap();
    let article_repository = MongoEntityRepository::<Article>::new(collections.articles.clone());
    ApiState {
        article_repository: Arc::new(article_repository.clone()),
    }
}

pub fn map_endpoints(state: ApiState) -> axum::Router {
    Router::new().nest(
        "/v1",
        Router::new().nest(
            "/article",
            Router::new()
                .merge(map_create_update_articles_v1_endpoint(state.clone()))
                .merge(map_get_article_v1_endpoint(state.clone()))
                .merge(map_get_article_list_v1_endpoint(state.clone()))
                .merge(map_delete_articles_v1_endpoint(state.clone())),
        ),
    )
}
