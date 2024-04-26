use crate::{
    dtos::article_dto::{CreateArticleDTO, ReadArticleDTO, UpdateArticleDTO},
    entities::article_entity::ArticleEntity,
    errors::ArticleError,
    repositories::{article_repository::ArticleRepositoryImpl, RepositoryBase},
};
use axum::async_trait;
use core::fmt::Debug;
use std::sync::Arc;

#[async_trait]
pub trait ArticleService: Send + Sync {
    // TODO: use dyn ArticleRepository
    fn new(article_repository: Arc<ArticleRepositoryImpl>) -> Self
    where
        Self: Sized;
    async fn insert_article(
        &self,
        article: CreateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError>;
    async fn get_article_by_id(&self, id: i32) -> Result<ReadArticleDTO, ArticleError>;
    async fn get_articles_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<Vec<ReadArticleDTO>, ArticleError>;
    async fn update_article(
        &self,
        id: i32,
        article: UpdateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError>;
    async fn delete_article(&self, id: i32) -> Result<(), ArticleError>;
}

#[derive(Debug, Clone, Default)]
pub struct ArticleServiceImpl {
    pub article_repository: Arc<ArticleRepositoryImpl>, // TODO: use dyn ArticleRepository
}

#[async_trait]
impl ArticleService for ArticleServiceImpl {
    // TODO: use dyn ArticleRepository
    fn new(article_repository: Arc<ArticleRepositoryImpl>) -> Self {
        Self {
            article_repository: article_repository,
        }
    }

    async fn insert_article(
        &self,
        article: CreateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError> {
        let mut entity = ArticleEntity::new();
        entity.set_title(article.title.clone())?;
        entity.set_content(article.content.clone())?;
        let result = self.article_repository.insert(entity).await;
        let dto = result.to_read_dto();
        Ok(dto)
    }

    async fn get_article_by_id(&self, id: i32) -> Result<ReadArticleDTO, ArticleError> {
        let article = self.article_repository.get_by_id(id).await;
        match article {
            Some(article) => Ok(article.to_read_dto()),
            None => Err(ArticleError::NotFoundError),
        }
    }

    async fn get_articles_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<Vec<ReadArticleDTO>, ArticleError> {
        let articles = self
            .article_repository
            .get_all_paginated(page_number, page_size)
            .await;
        let dtos = articles
            .iter()
            .map(|article| article.to_read_dto())
            .collect();
        Ok(dtos)
    }

    async fn update_article(
        &self,
        id: i32,
        article: UpdateArticleDTO,
    ) -> Result<ReadArticleDTO, ArticleError> {
        let existing_article = self.article_repository.get_by_id(id).await;
        if existing_article.is_none() {
            return Err(ArticleError::NotFoundError);
        }
        let mut existing_article = existing_article.unwrap();
        existing_article.set_title(article.title)?;
        existing_article.set_content(article.content)?;
        let updated_article = self.article_repository.update(existing_article).await;
        Ok(updated_article.to_read_dto())
    }

    async fn delete_article(&self, id: i32) -> Result<(), ArticleError> {
        let existing_article = self.article_repository.get_by_id(id).await;
        if existing_article.is_none() {
            return Err(ArticleError::NotFoundError);
        }
        self.article_repository.delete(id).await;
        Ok(())
    }
}
