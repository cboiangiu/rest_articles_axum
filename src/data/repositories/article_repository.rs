use super::{
    repository_base::{RepositoryBase, RepositoryBaseImpl},
    PaginatedResult,
};
use crate::entities::article::Article;
use axum::async_trait;
use bson::oid::ObjectId;
use mongodb::Collection;

#[async_trait]
pub trait ArticleRepository: RepositoryBase<Article> {}

#[derive(Debug, Clone)]
pub struct ArticleRepositoryImpl {
    repository_base: RepositoryBaseImpl<Article>,
    #[allow(dead_code)]
    collection: Collection<Article>,
}

impl ArticleRepositoryImpl {
    pub fn new(collection: Collection<Article>) -> Self {
        Self {
            repository_base: RepositoryBaseImpl {
                collection: collection.clone(),
            },
            collection: collection.clone(),
        }
    }
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {}

#[async_trait]
impl RepositoryBase<Article> for ArticleRepositoryImpl {
    async fn insert(&self, entity: Article) -> Result<Article, mongodb::error::Error> {
        self.repository_base.insert(entity).await
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
        self.repository_base.update(entity).await
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        self.repository_base.delete(id).await
    }
}
