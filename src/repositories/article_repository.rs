use super::{RepositoryBase, RepositoryBaseImpl};
use crate::entities::article_entity::ArticleEntity;
use axum::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait ArticleRepository: RepositoryBase<ArticleEntity> {}

#[derive(Debug, Clone, Default)]
pub struct ArticleRepositoryImpl {
    repository_base: RepositoryBaseImpl<ArticleEntity>,
}

#[async_trait]
impl ArticleRepository for ArticleRepositoryImpl {}

#[async_trait]
impl RepositoryBase<ArticleEntity> for ArticleRepositoryImpl {
    fn new(entities: Arc<Mutex<Vec<ArticleEntity>>>) -> Self {
        Self {
            repository_base: RepositoryBaseImpl::new(entities.clone()),
        }
    }

    async fn insert(&self, entity: ArticleEntity) -> ArticleEntity {
        self.repository_base.insert(entity).await
    }

    async fn get_by_id(&self, id: i32) -> Option<ArticleEntity> {
        self.repository_base.get_by_id(id).await
    }

    async fn get_all(&self) -> Vec<ArticleEntity> {
        self.repository_base.get_all().await
    }

    async fn get_all_paginated(&self, page_number: usize, page_size: usize) -> Vec<ArticleEntity> {
        self.repository_base
            .get_all_paginated(page_number, page_size)
            .await
    }

    async fn update(&self, entity: ArticleEntity) -> ArticleEntity {
        self.repository_base.update(entity).await
    }

    async fn delete(&self, id: i32) {
        self.repository_base.delete(id).await
    }
}
