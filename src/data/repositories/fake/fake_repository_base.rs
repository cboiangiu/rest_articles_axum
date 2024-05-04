use crate::{
    data::repositories::{repository_base::RepositoryBase, PaginatedResult},
    entities::EntityWithId,
};
use axum::async_trait;
use bson::oid::ObjectId;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct FakeRepositoryBase<T> {
    articles: Arc<Mutex<Vec<T>>>,
}

impl<T: EntityWithId> FakeRepositoryBase<T> {
    pub fn new(articles: Arc<Mutex<Vec<T>>>) -> Self {
        Self { articles }
    }
}

#[async_trait]
impl<T: EntityWithId> RepositoryBase<T> for FakeRepositoryBase<T> {
    async fn insert(&self, entity: T) -> Result<T, mongodb::error::Error> {
        let mut entity = entity;
        entity.set_id(ObjectId::new());
        self.articles.lock().unwrap().push(entity.clone());
        Ok(entity)
    }

    async fn get_by_id(&self, id: &ObjectId) -> Result<Option<T>, mongodb::error::Error> {
        let articles = self.articles.lock().unwrap().clone();
        let article = articles.iter().find(|a| a.get_id() == id);
        match article {
            Some(a) => Ok(Some(a.clone())),
            None => Ok(None),
        }
    }

    async fn get_all(&self) -> Result<Vec<T>, mongodb::error::Error> {
        Ok(self.articles.lock().unwrap().clone())
    }

    async fn get_all_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedResult<T>, mongodb::error::Error> {
        let page_number = page_number.max(1);
        let page_size: usize = page_size.max(10);
        let start = page_number * page_size;
        let end = start + page_size;
        let articles = self.articles.lock().unwrap()[start..end].to_vec();
        Ok(PaginatedResult {
            data: articles,
            total: self.articles.lock().unwrap().len() as u64,
        })
    }

    async fn update(&self, entity: T) -> Result<T, mongodb::error::Error> {
        let index = self
            .articles
            .lock()
            .unwrap()
            .iter()
            .position(|a| a.get_id() == entity.get_id())
            .unwrap();
        self.articles.lock().unwrap()[index] = entity.clone();
        Ok(entity)
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        let index = self
            .articles
            .lock()
            .unwrap()
            .iter()
            .position(|a| a.get_id() == id)
            .unwrap();
        self.articles.lock().unwrap().remove(index);
        Ok(())
    }
}
