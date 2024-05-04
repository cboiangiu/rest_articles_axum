use super::PaginatedResult;
use crate::entities::EntityWithId;
use axum::async_trait;
use bson::{doc, oid::ObjectId};
use core::fmt::Debug;
use futures_util::stream::TryStreamExt;
use mongodb::{options::FindOptions, Collection};
use std::sync::Arc;

#[async_trait]
pub trait RepositoryBase<T: EntityWithId>: Send + Sync + Debug {
    async fn insert(&self, entity: T) -> Result<T, mongodb::error::Error>;
    async fn get_by_id(&self, id: &ObjectId) -> Result<Option<T>, mongodb::error::Error>;
    async fn get_all(&self) -> Result<Vec<T>, mongodb::error::Error>;
    async fn get_all_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedResult<T>, mongodb::error::Error>;
    async fn update(&self, entity: T) -> Result<T, mongodb::error::Error>;
    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error>;
}

#[derive(Debug, Clone)]
pub struct RepositoryBaseImpl<T: EntityWithId> {
    pub collection: Collection<T>,
}

#[async_trait]
impl<T: EntityWithId> RepositoryBase<T> for RepositoryBaseImpl<T> {
    async fn insert(&self, entity: T) -> Result<T, mongodb::error::Error> {
        let insert_result = self.collection.insert_one(entity.clone(), None).await?;
        let mut entity = entity;
        entity.set_id(insert_result.inserted_id.as_object_id().unwrap().clone());
        Ok(entity)
    }

    async fn get_by_id(&self, id: &ObjectId) -> Result<Option<T>, mongodb::error::Error> {
        let filter = doc! {"_id": id };
        self.collection.find_one(filter, None).await
    }

    async fn get_all(&self) -> Result<Vec<T>, mongodb::error::Error> {
        let cursor = self.collection.find(None, None).await?;
        cursor.try_collect().await
    }

    async fn get_all_paginated(
        &self,
        page_number: usize,
        page_size: usize,
    ) -> Result<PaginatedResult<T>, mongodb::error::Error> {
        let page_number = page_number.max(1);
        let skip = (page_number - 1) * page_size;
        let limit = page_size.max(10);

        let options = FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .build();

        let cursor = self.collection.find(None, options).await?;
        let results = cursor.try_collect().await;
        let total = self.collection.count_documents(None, None).await?;
        results.map(|data| PaginatedResult {
            data: data,
            total: total,
        })
    }

    async fn update(&self, entity: T) -> Result<T, mongodb::error::Error> {
        let update_result = self
            .collection
            .update_one(
                doc! { "_id": entity.get_id() },
                doc! { "$set": bson::to_document(&entity).unwrap() },
                None,
            )
            .await?;

        if update_result.matched_count == 1 {
            Ok(entity)
        } else {
            Err(mongodb::error::Error::from(
                mongodb::error::ErrorKind::Custom(Arc::new("Something went wrong")),
            ))
        }
    }

    async fn delete(&self, id: &ObjectId) -> Result<(), mongodb::error::Error> {
        let delete_result = self.collection.delete_one(doc! { "_id": id }, None).await?;
        if delete_result.deleted_count == 1 {
            Ok(())
        } else {
            Err(mongodb::error::Error::from(
                mongodb::error::ErrorKind::Custom(Arc::new("Something went wrong")),
            ))
        }
    }
}
