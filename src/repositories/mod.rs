pub mod article_repository;
use crate::entities::EntityWithId;
use axum::async_trait;
use core::fmt::Debug;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait RepositoryBase<T: EntityWithId>: Send + Sync + Debug + Default {
    fn new(entities: Arc<Mutex<Vec<T>>>) -> Self
    where
        Self: Sized;
    async fn insert(&self, entity: T) -> T;
    async fn get_by_id(&self, id: i32) -> Option<T>;
    async fn get_all(&self) -> Vec<T>;
    async fn get_all_paginated(&self, page_number: usize, page_size: usize) -> Vec<T>;
    async fn update(&self, entity: T) -> T;
    async fn delete(&self, id: i32) -> ();
}

#[derive(Debug, Clone, Default)]
pub struct RepositoryBaseImpl<T> {
    pub entities: Arc<Mutex<Vec<T>>>,
}

#[async_trait]
impl<T: EntityWithId + Send + Sync + Debug + Clone + Default> RepositoryBase<T>
    for RepositoryBaseImpl<T>
{
    fn new(entities: Arc<Mutex<Vec<T>>>) -> Self {
        Self {
            entities: entities.clone(),
        }
    }

    async fn insert(&self, entity: T) -> T {
        let mut entities = self.entities.lock().await;
        let mut entity = entity.clone();
        entity.set_id(entities.len() as i32 + 1);
        entities.push(entity.clone());
        entity
    }

    async fn get_by_id(&self, id: i32) -> Option<T> {
        let entities = self.entities.lock().await;
        entities
            .iter()
            .find(|entity| entity.get_id() == id)
            .cloned()
    }

    async fn get_all(&self) -> Vec<T> {
        let entities = self.entities.lock().await;
        entities.clone()
    }

    async fn get_all_paginated(&self, page_number: usize, page_size: usize) -> Vec<T> {
        let entities = self.entities.lock().await;
        let page_number = page_number.max(1);
        let start_index = (page_number - 1) * page_size;
        let end_index = start_index + page_size;
        if start_index >= entities.len() {
            return vec![];
        }
        if end_index >= entities.len() {
            return entities[start_index..].to_vec();
        }
        entities[start_index as usize..end_index as usize].to_vec()
    }

    async fn update(&self, entity: T) -> T {
        let mut entities = self.entities.lock().await;
        let index = entities
            .iter()
            .position(|e| e.get_id() == entity.get_id())
            .unwrap();
        entities[index] = entity.clone();
        entity
    }

    async fn delete(&self, id: i32) -> () {
        let mut entities = self.entities.lock().await;
        let index = entities.iter().position(|e| e.get_id() == id).unwrap();
        entities.remove(index);
    }
}
