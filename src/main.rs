mod domain;
mod framework;
mod modules;
use crate::framework::infrastructure::open_api::ApiDoc;
use crate::modules::api;
use crate::modules::web;
use axum::{serve, Router};
// use modules::api::features::articles::article::get_article_v1::proto_get_article::get_article_v1_server::GetArticleV1Server;
// use modules::api::features::articles::article::get_article_v1::GetArticleV1Grpc;
// use modules::api::ApiModuleState;
// use tonic::transport::Server;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// mod proto {
//     pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
//         tonic::include_file_descriptor_set!("rest_articles_axum_features_descriptor");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let web_module_state = web::new_web_module_state().await;
    let api_module_state = api::new_api_module_state().await;

    let app = Router::new()
        .nest("/", web::map_endpoints(web_module_state.clone()))
        .nest("/api", api::map_endpoints(api_module_state.clone()))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CorsLayer::permissive())
        .layer(CatchPanicLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(CompressionLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // let service2222 = tonic_reflection::server::Builder::configure()
    //     .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    //     .build()
    //     .unwrap();
    // let getArticleV1Grpc = GetArticleV1Grpc {
    //     state: api_module_state.clone(),
    // };
    // Server::builder()
    //     .add_service(service2222)
    //     .add_service(GetArticleV1Server::new(getArticleV1Grpc))
    //     .serve(addr)
    //     .await?;

    println!("Server listening on http://{}", addr);
    println!("Web available at http://{}", addr);
    println!("Swagger available at http://{}/swagger-ui", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();

    Ok(())
}
