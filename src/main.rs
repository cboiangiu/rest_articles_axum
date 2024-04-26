mod controllers;
mod dtos;
mod entities;
mod errors;
mod repositories;
mod services;
use crate::dtos::article_dto::{CreateArticleDTO, ReadArticleDTO, UpdateArticleDTO};
use crate::errors::ArticleError;
use crate::{
    controllers::article_controller::{self, ArticleControllerState},
    entities::{article_entity::ArticleEntity, BaseEntity},
    repositories::{article_repository::ArticleRepositoryImpl, RepositoryBase},
    services::article_service::{ArticleService, ArticleServiceImpl},
};
use axum::{serve, Router};
use chrono::{DateTime, Utc};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            article_controller::insert_article,
            article_controller::get_article_by_id,
            article_controller::get_articles,
            article_controller::update_article,
            article_controller::delete_article,
        ),
        components(schemas(CreateArticleDTO, ReadArticleDTO, UpdateArticleDTO, ArticleError))
    )]
    struct ApiDoc;

    let cors_layer = CorsLayer::permissive();

    let articles: Arc<Mutex<Vec<ArticleEntity>>> = Arc::new(Mutex::new(vec![
        ArticleEntity {
            base: BaseEntity { id: 1 },
            title: "Article One".to_string(),
            content: "Some content".to_string(),
            published_date: DateTime::from(Utc::now()),
        },
        ArticleEntity {
            base: BaseEntity { id: 2 },
            title: "Another Article".to_string(),
            content: "More content".to_string(),
            published_date: DateTime::from(Utc::now()),
        },
    ]));

    let article_repository = ArticleRepositoryImpl::new(Arc::clone(&articles));
    let article_service = ArticleServiceImpl::new(Arc::new(article_repository.clone()));

    let article_controller_state = ArticleControllerState {
        article_service: Arc::new(article_service.clone()),
    };

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest(
            "/v1",
            Router::new().merge(article_controller::route(article_controller_state)),
        )
        .layer(cors_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);
    println!("Swagger available at http://{}/swagger-ui", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();

    Ok(())
}
