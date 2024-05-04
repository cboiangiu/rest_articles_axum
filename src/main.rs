mod controllers;
mod data;
mod dtos;
mod entities;
mod errors;
mod services;
use crate::{
    controllers::{
        article_controller::{self, ArticleControllerState},
        ApiDoc,
    },
    data::repositories::article_repository::ArticleRepositoryImpl,
    services::article_service::ArticleServiceImpl,
};
use axum::{serve, Router};
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let collections = data::connect_to_mongo().await.unwrap();
    let article_repository = ArticleRepositoryImpl::new(collections.articles.clone());
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
        .layer(CorsLayer::permissive())
        .layer(CatchPanicLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CompressionLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);
    println!("Swagger available at http://{}/swagger-ui", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();

    Ok(())
}
