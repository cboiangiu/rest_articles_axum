use crate::modules::{
    api::{features::articles::article::get_article_list_v1, persistence::ArticleRepository},
    web::{
        templates::{ArticleComponent, ArticleListComponent, IndexPage},
        WebModuleState,
    },
};
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use std::{sync::Arc, vec};

pub fn map_index_page_endpoint(state: WebModuleState) -> Router {
    Router::new()
        .route(
            "/",
            get(|State(state): State<WebModuleState>| index_page_handler(state.article_repository)),
        )
        .with_state(state)
}

async fn index_page_handler(repository: Arc<dyn ArticleRepository>) -> impl IntoResponse {
    let api_query = get_article_list_v1::GetArticleListQuery {
        page_number: 1,
        page_size: 10,
    };
    let response = get_article_list_v1::get_article_list_handler(repository, api_query).await;
    let mut items = vec![];
    match response {
        Ok(value) => items = value.1 .0.items,
        _ => (),
    }

    let mut article_components: Vec<ArticleComponent> = vec![];
    for item in items {
        article_components.push(item.into());
    }

    IndexPage {
        article_list_component: ArticleListComponent {
            article_list: article_components,
        },
    }
}
