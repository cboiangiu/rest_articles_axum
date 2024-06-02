use crate::framework::core::errors::map_api_error;
use crate::{
    domain::articles::article::Article,
    framework::core::{domain::EntityWithId, errors::ApiError},
    modules::api::{persistence::ArticleRepository, ApiModuleState},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tonic::async_trait;
use tonic::{Request, Response, Status};
use utoipa::ToSchema;
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
}
pub use proto::api::articles::article::get_article::v1 as proto_get_article;

pub fn map_get_article_v1_endpoint(state: ApiModuleState) -> Router {
    Router::new()
        .route("/:id", get(get_article_v1_endpoint))
        .with_state(state)
}

#[utoipa::path(
    get,
    path = "/api/v1/article/{id}",   
    tag = "article",
    params(
        ("id" = String, Path,)
    ),
    responses(
        (status = 200, body = GetArticleResponse),
        (status = 404, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn get_article_v1_endpoint(
    State(state): State<ApiModuleState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<GetArticleResponse>), ApiError> {
    get_article_handler(state.article_repository, GetArticleQuery { id }).await
}

pub struct GetArticleV1Grpc {
    pub state: ApiModuleState,
}

#[async_trait]
impl proto_get_article::get_article_v1_server::GetArticleV1 for GetArticleV1Grpc {
    async fn handler(
        &self,
        query: Request<proto_get_article::GetArticleQuery>,
    ) -> Result<Response<proto_get_article::GetArticleResponse>, Status> {
        let item = get_article_handler(
            self.state.article_repository.clone(),
            query.into_inner().into(),
        )
        .await
        .map_err(map_api_error)?
        .1
         .0;
        Ok(Response::new(item.into()))
    }
}

pub async fn get_article_handler(
    repository: Arc<dyn ArticleRepository>,
    query: GetArticleQuery,
) -> Result<(StatusCode, Json<GetArticleResponse>), ApiError> {
    if ObjectId::parse_str(query.id.clone()).is_err() {
        return Err(ApiError::InvalidObjectIdError);
    }
    let item = repository.base().get_by_id(query.id).await?;
    match item {
        Some(item) => {
            return Ok((StatusCode::OK, Json(item.into())));
        }
        None => {
            return Err(ApiError::NotFoundError);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, o2o)]
#[from_owned(proto_get_article::GetArticleQuery)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleQuery {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, o2o)]
#[from_owned(Article)]
#[owned_into(proto_get_article::GetArticleResponse)]
#[serde(rename_all = "camelCase")]
pub struct GetArticleResponse {
    // #[ghost(@.get_id().to_hex())]
    #[from(@.get_id().to_hex())]
    pub id: String,
    pub title: String,
    pub content: String,
    #[into(~.to_string())]
    pub published_date: DateTime<Utc>,
}
