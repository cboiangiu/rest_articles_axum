use super::fake_repository_base::FakeRepositoryBase;
use crate::{
    data::repositories::{
        article_repository::ArticleRepository, repository_base::RepositoryBase, PaginatedResult,
    },
    entities::article::Article,
};
use axum::async_trait;
use bson::oid::ObjectId;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct FakeArticleRepository {
    repository_base: FakeRepositoryBase<Article>,
    #[allow(dead_code)]
    articles: Arc<Mutex<Vec<Article>>>,
}

impl FakeArticleRepository {
    #[allow(dead_code)]
    pub fn new(articles: Arc<Mutex<Vec<Article>>>) -> Self {
        Self {
            repository_base: FakeRepositoryBase::new(articles.clone()),
            articles: articles.clone(),
        }
    }
}

#[async_trait]
impl ArticleRepository for FakeArticleRepository {}

#[async_trait]
impl RepositoryBase<Article> for FakeArticleRepository {
    async fn insert(&self, entity: Article) -> Result<Article, mongodb::error::Error> {
        self.repository_base.insert(entity.clone()).await
    }

    async fn get_by_id(&self, id: &ObjectId) -> Result<Option<Article>, mongodb::error::Error> {
        self.repository_base.get_by_id(id).await
    }

    async fn get_all(&self) -> Result<Vec<Article>, mongodb::error::Error> {
        self.repository_base.get_all().await
    }

    async fn get_all_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedResult<Article>, mongodb::error::Error> {
        self.repository_base
            .get_all_paginated(page_number, page_size)
            .await
    }

    async fn update(&self, entity: Article) -> Result<Article, mongodb::error::Error> {
        self.repository_base.update(entity.clone()).await
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.repository_base.delete(id).await
    }
}
