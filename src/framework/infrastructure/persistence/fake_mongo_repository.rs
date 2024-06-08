// use super::{
//     mongo_repository::{MongoRepository, WithMongoRepository},
//     PaginatedResult,
// };
// use crate::framework::core::{domain::EntityWithId, errors::ApiError};
// use axum::async_trait;
// use bson::oid::ObjectId;
// use core::fmt::Debug;
// use std::sync::{Arc, Mutex};

// #[derive(Debug, Clone)]
// pub struct FakeMongoEntityRepository<T: EntityWithId> {
//     repository_base: FakeMongoRepositoryBase<T>,
//     #[allow(dead_code)]
//     articles: Arc<Mutex<Vec<T>>>,
// }

// #[async_trait]
// impl<T: EntityWithId> WithMongoRepository<T> for FakeMongoEntityRepository<T>
// where
//     T: EntityWithId + 'static,
// {
//     fn base(&self) -> Box<dyn MongoRepository<T>> {
//         Box::new(self.repository_base.clone())
//     }
// }

// impl<T: EntityWithId> FakeMongoEntityRepository<T> {
//     #[allow(dead_code)]
//     pub fn new(articles: Arc<Mutex<Vec<T>>>) -> Self {
//         Self {
//             repository_base: FakeMongoRepositoryBase::new(articles.clone()),
//             articles: articles.clone(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct FakeMongoRepositoryBase<T> {
//     articles: Arc<Mutex<Vec<T>>>,
// }

// impl<T: EntityWithId> FakeMongoRepositoryBase<T> {
//     pub fn new(articles: Arc<Mutex<Vec<T>>>) -> Self {
//         Self { articles }
//     }
// }

// #[async_trait]
// impl<T: EntityWithId> MongoRepository<T> for FakeMongoRepositoryBase<T> {
//     async fn insert(&self, entity: T) -> Result<T, ApiError> {
//         let mut entity = entity;
//         entity.set_id(ObjectId::new());
//         self.articles.lock().unwrap().push(entity.clone());
//         Ok(entity)
//     }

//     async fn get_by_id(&self, id: String) -> Result<Option<T>, ApiError> {
//         if ObjectId::parse_str(id.clone()).is_err() {
//             return Err(ApiError::InvalidObjectIdError);
//         }
//         let id = ObjectId::parse_str(id).unwrap();
//         let articles = self.articles.lock().unwrap().clone();
//         let article = articles.iter().find(|a| a.get_id().to_owned() == id);
//         match article {
//             Some(a) => Ok(Some(a.clone())),
//             None => Ok(None),
//         }
//     }

//     async fn get_all(&self) -> Result<Vec<T>, ApiError> {
//         Ok(self.articles.lock().unwrap().clone())
//     }

//     async fn get_all_paginated(
//         &self,
//         page_number: usize,
//         page_size: usize,
//     ) -> Result<PaginatedResult<T>, ApiError> {
//         let page_number = page_number.max(1);
//         let page_size: usize = page_size.max(10);
//         let start = page_number * page_size;
//         let end = start + page_size;
//         let articles = self.articles.lock().unwrap()[start..end].to_vec();
//         Ok(PaginatedResult {
//             data: articles,
//             total: self.articles.lock().unwrap().len() as u64,
//         })
//     }

//     async fn update(&self, entity: T) -> Result<T, ApiError> {
//         let index = self
//             .articles
//             .lock()
//             .unwrap()
//             .iter()
//             .position(|a| a.get_id() == entity.get_id())
//             .unwrap();
//         self.articles.lock().unwrap()[index] = entity.clone();
//         Ok(entity)
//     }

//     async fn delete(&self, id: String) -> Result<(), ApiError> {
//         if ObjectId::parse_str(id.clone()).is_err() {
//             return Err(ApiError::InvalidObjectIdError);
//         }
//         let id = ObjectId::parse_str(id).unwrap();
//         let index = self
//             .articles
//             .lock()
//             .unwrap()
//             .iter()
//             .position(|a| a.get_id().to_owned() == id)
//             .unwrap();
//         self.articles.lock().unwrap().remove(index);
//         Ok(())
//     }
// }
