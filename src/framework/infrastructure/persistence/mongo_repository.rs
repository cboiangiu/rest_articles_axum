use super::PaginatedResult;
use crate::framework::core::{domain::EntityWithId, errors::ApiError};
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use core::fmt::Debug;
use futures_util::stream::TryStreamExt;
use mongodb::{options::FindOptions, Collection};

pub trait WithMongoRepository<T: EntityWithId> {
    fn base(&self) -> Box<dyn MongoRepository<T>>;
}

#[derive(Debug, Clone)]
pub struct MongoEntityRepository<T: EntityWithId> {
    pub repository_base: MongoRepositoryBase<T>,
    #[allow(dead_code)]
    collection: Collection<T>,
}

#[async_trait]
impl<T: EntityWithId> WithMongoRepository<T> for MongoEntityRepository<T>
where
    T: EntityWithId + 'static,
{
    fn base(&self) -> Box<dyn MongoRepository<T>> {
        Box::new(self.repository_base.clone())
    }
}

impl<T: EntityWithId> MongoEntityRepository<T> {
    pub fn new(collection: Collection<T>) -> Self {
        Self {
            repository_base: MongoRepositoryBase {
                collection: collection.clone(),
            },
            collection: collection.clone(),
        }
    }
}

#[async_trait]
pub trait MongoRepository<T: EntityWithId>: Send + Sync + Debug {
    async fn insert(&self, entity: T) -> Result<T, ApiError>;
    async fn get_by_id(&self, id: String) -> Result<Option<T>, ApiError>;
    #[allow(dead_code)]
    async fn get_all(&self) -> Result<Vec<T>, ApiError>;
    async fn get_all_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedResult<T>, ApiError>;
    async fn update(&self, entity: T) -> Result<T, ApiError>;
    async fn delete(&self, id: String) -> Result<(), ApiError>;
}

#[derive(Debug, Clone)]
pub struct MongoRepositoryBase<T: EntityWithId> {
    pub collection: Collection<T>,
}

#[async_trait]
impl<T: EntityWithId> MongoRepository<T> for MongoRepositoryBase<T> {
    async fn insert(&self, entity: T) -> Result<T, ApiError> {
        let insert_result = self
            .collection
            .insert_one(entity.clone(), None)
            .await
            .map_err(map_mongo_err)?;

        let mut entity = entity;
        entity.set_id(insert_result.inserted_id.as_object_id().unwrap().clone());
        Ok(entity)
    }

    async fn get_by_id(&self, id: String) -> Result<Option<T>, ApiError> {
        if ObjectId::parse_str(id.clone()).is_err() {
            return Err(ApiError::InvalidObjectIdError);
        }
        let id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": id };
        Ok(self
            .collection
            .find_one(filter, None)
            .await
            .map_err(map_mongo_err)?)
    }

    async fn get_all(&self) -> Result<Vec<T>, ApiError> {
        let cursor = self
            .collection
            .find(None, None)
            .await
            .map_err(map_mongo_err)?;

        Ok(cursor.try_collect().await.map_err(map_mongo_err)?)
    }

    async fn get_all_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedResult<T>, ApiError> {
        let page_number = page_number.max(1);
        let skip = (page_number - 1) * page_size;
        let limit = page_size.max(10);

        let options = FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .build();

        let cursor = self
            .collection
            .find(None, options)
            .await
            .map_err(map_mongo_err)?;

        let results = cursor.try_collect().await;
        let total = self
            .collection
            .count_documents(None, None)
            .await
            .map_err(map_mongo_err)?;

        Ok(results
            .map(|data| PaginatedResult {
                data: data,
                total: total,
            })
            .map_err(map_mongo_err)?)
    }

    async fn update(&self, entity: T) -> Result<T, ApiError> {
        let update_result = self
            .collection
            .update_one(
                doc! { "_id": entity.get_id() },
                doc! { "$set": bson::to_document(&entity).unwrap() },
                None,
            )
            .await
            .map_err(map_mongo_err)?;

        if update_result.matched_count == 1 {
            return Ok(entity);
        }
        Err(ApiError::InternalServerError)
    }

    async fn delete(&self, id: String) -> Result<(), ApiError> {
        if ObjectId::parse_str(id.clone()).is_err() {
            return Err(ApiError::InvalidObjectIdError);
        }
        let id = ObjectId::parse_str(id).unwrap();
        let delete_result = self
            .collection
            .delete_one(doc! { "_id": id }, None)
            .await
            .map_err(map_mongo_err)?;
        if delete_result.deleted_count == 1 {
            return Ok(());
        }
        Err(ApiError::InternalServerError)
    }
}

pub fn map_mongo_err(err: mongodb::error::Error) -> ApiError {
    match err {
        _ => ApiError::InternalServerError,
    }
}
