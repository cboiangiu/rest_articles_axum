use crate::{
    framework::core::{domain::EntityWithId, errors::ApiError, CreateUpdateEntitiesResponse},
    modules::api::{
        domain::articles::article::Article, persistence::ArticleRepository, ApiModuleState,
    },
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::put, Json, Router};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

pub fn map_create_update_articles_v1_endpoint(state: ApiModuleState) -> Router {
    Router::new()
        .route("/", put(create_update_articles_v1_endpoint))
        .with_state(state)
}

#[utoipa::path(
    put,
    path = "/api/v1/article",   
    tag = "article",
    request_body = CreateUpdateArticlesCommand,
    responses(
        (status = 201, content(("application/simple.response+json" = CreateUpdateEntitiesResponse), ("application/json" = CreateUpdateArticlesResponse))),
        (status = 200, content(("application/simple.response+json" = CreateUpdateEntitiesResponse), ("application/json" = CreateUpdateArticlesResponse))),
        (status = 400, body = ErrorResponse),
        (status = 500, body = ErrorResponse)
    )
)]
async fn create_update_articles_v1_endpoint(
    State(state): State<ApiModuleState>,
    request: Json<CreateUpdateArticlesCommand>,
) -> Result<(StatusCode, impl IntoResponse), ApiError> {
    create_update_articles_handler(state.article_repository, request.0).await
}

pub async fn create_update_articles_handler(
    repository: Arc<dyn ArticleRepository>,
    command: CreateUpdateArticlesCommand,
) -> Result<(StatusCode, impl IntoResponse), ApiError> {
    create_update_articles_validator(&command)?;

    let mut articles_created: Vec<Article> = vec![];
    let mut articles_updated: Vec<Article> = vec![];

    for article in command.articles {
        if article.id.is_none() {
            let entity = Article::create(article.title.clone(), article.content.clone())?;
            let result = repository.base().insert(entity).await?;
            articles_created.push(result.into());
            continue;
        }

        let existing_article = repository
            .base()
            .get_by_id(article.id.unwrap().clone())
            .await?;
        if existing_article.is_none() {
            return Err(ApiError::NotFoundError);
        }
        let existing_article = existing_article
            .unwrap()
            .update(article.title.clone(), article.content.clone())?;
        let updated_article = repository.base().update(existing_article).await?;
        articles_updated.push(updated_article.into());
    }
    let response_status_code = if articles_created.len() > 0 && articles_updated.len() == 0 {
        StatusCode::OK
    } else {
        StatusCode::CREATED
    };

    let singular_response_entity = if articles_created.len() == 1 && articles_updated.len() == 0 {
        Some(articles_created.first().unwrap())
    } else if articles_created.len() == 0 && articles_updated.len() == 1 {
        Some(articles_updated.first().unwrap())
    } else {
        None
    };
    if let Some(entity) = singular_response_entity {
        let response: CreateUpdateArticlesResponse = entity.clone().into();
        return Ok((response_status_code, Json(response).into_response()));
    }

    Ok((
        response_status_code,
        Json(CreateUpdateEntitiesResponse {
            entities_created: articles_created.len(),
            entities_updated: articles_updated.len(),
        })
        .into_response(),
    ))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateArticleCommand {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateArticlesCommand {
    pub articles: Vec<CreateUpdateArticleCommand>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, o2o)]
#[from_owned(Article)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateArticlesResponse {
    #[ghost(@.get_id().to_hex())]
    pub id: String,
    pub title: String,
    pub content: String,
    pub published_date: DateTime<Utc>,
}

fn create_update_articles_validator(command: &CreateUpdateArticlesCommand) -> Result<(), ApiError> {
    if command.articles.is_empty() {
        return Err(ApiError::ArgumentNullError("articles".to_string()));
    }
    for article in command.articles.iter() {
        create_update_article_validator(article)?
    }
    Ok(())
}

fn create_update_article_validator(command: &CreateUpdateArticleCommand) -> Result<(), ApiError> {
    if let Some(id) = command.id.clone() {
        if ObjectId::parse_str(id.clone()).is_err() {
            return Err(ApiError::InvalidObjectIdError);
        }
    }
    Ok(())
}
