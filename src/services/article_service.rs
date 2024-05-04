use crate::{
    data::repositories::article_repository::ArticleRepository,
    dtos::{
        article_dto::{CreateArticleDTO, ReadArticleDTO, UpdateArticleDTO},
        PaginatedDTO,
    },
    entities::article::Article,
    errors::ArticleError,
};
use axum::async_trait;
use bson::oid::ObjectId;
use core::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait ArticleService: Send + Sync {
    async fn insert_article(
        &self,
        article: CreateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError>;
    async fn get_article_by_id(&self, id: String) -> Result<ReadArticleDTO, ArticleError>;
    async fn get_articles_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedDTO<ReadArticleDTO>, ArticleError>;
    async fn update_article(
        &self,
        id: String,
        article: UpdateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError>;
    async fn delete_article(&self, id: String) -> Result<(), ArticleError>;
}

#[derive(Debug, Clone)]
pub struct ArticleServiceImpl {
    pub article_repository: Arc<dyn ArticleRepository>,
}

impl ArticleServiceImpl {
    pub fn new(article_repository: Arc<dyn ArticleRepository>) -> Self {
        Self {
            article_repository: article_repository,
        }
    }
}

#[async_trait]
impl ArticleService for ArticleServiceImpl {
    async fn insert_article(
        &self,
        article: CreateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError> {
        let mut entity = Article::new();
        entity.set_title(article.title.clone())?;
        entity.set_content(article.content.clone())?;
        let result = self.article_repository.insert(entity).await;
        Ok(result.unwrap().into())
    }

    async fn get_article_by_id(&self, id: String) -> Result<ReadArticleDTO, ArticleError> {
        if ObjectId::parse_str(id.clone()).is_err() {
            return Err(ArticleError::InvalidObjectIdError);
        }
        let article = self
            .article_repository
            .get_by_id(ObjectId::parse_str(id).as_ref().unwrap())
            .await
            .unwrap();
        match article {
            Some(article) => Ok(article.into()),
            None => Err(ArticleError::NotFoundError),
        }
    }

    async fn get_articles_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedDTO<ReadArticleDTO>, ArticleError> {
        let articles = self
            .article_repository
            .get_all_paginated(page_number, page_size)
            .await
            .unwrap();
        let dtos = PaginatedDTO {
            items: articles
                .data
                .iter()
                .map(|article| article.clone().into())
                .collect(),
            total_items: articles.total,
            page_number,
            page_size,
        };
        Ok(dtos)
    }

    async fn update_article(
        &self,
        id: String,
        article: UpdateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError> {
        if ObjectId::parse_str(id.clone()).is_err() {
            return Err(ArticleError::InvalidObjectIdError);
        }
        let existing_article = self
            .article_repository
            .get_by_id(ObjectId::parse_str(id).as_ref().unwrap())
            .await
            .unwrap();
        if existing_article.is_none() {
            return Err(ArticleError::NotFoundError);
        }
        let mut existing_article = existing_article.unwrap();
        existing_article.set_title(article.title)?;
        existing_article.set_content(article.content)?;
        let updated_article = self.article_repository.update(existing_article).await;
        Ok(updated_article.unwrap().into())
    }

    async fn delete_article(&self, id: String) -> Result<(), ArticleError> {
        if ObjectId::parse_str(id.clone()).is_err() {
            return Err(ArticleError::InvalidObjectIdError);
        }
        let existing_article = self
            .article_repository
            .get_by_id(ObjectId::parse_str(id.clone()).as_ref().unwrap())
            .await
            .unwrap();
        if existing_article.is_none() {
            return Err(ArticleError::NotFoundError);
        }
        self.article_repository
            .delete(ObjectId::parse_str(id.clone()).as_ref().unwrap())
            .await
            .unwrap();
        Ok(())
    }
}
