use crate::framework::core::{
    domain::{BaseEntity, EntityWithId},
    errors::ApiError,
};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Article {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub title: String,
    pub content: String,
    pub published_date: DateTime<Utc>,
}

impl EntityWithId for Article {
    fn get_id(&self) -> &ObjectId {
        self.base.id.as_ref().unwrap()
    }
    fn set_id(&mut self, id: ObjectId) {
        self.base.id = Some(id);
    }
}

impl Article {
    fn validate(&self) -> Result<(), ApiError> {
        if self.title.trim().is_empty() {
            return Err(ApiError::ArgumentNullError("title".to_string()));
        }
        if self.title.len() > Self::TITLE_MAX_LENGTH {
            return Err(ApiError::TextTooLongError("title".to_string()));
        }
        if self.content.trim().is_empty() {
            return Err(ApiError::ArgumentNullError("content".to_string()));
        }
        if self.content.len() > Self::CONTENT_MAX_LENGTH {
            return Err(ApiError::TextTooLongError("content".to_string()));
        }
        Ok(())
    }

    pub fn create(title: String, content: String) -> Result<Self, ApiError> {
        let entity = Self {
            base: BaseEntity { id: None },
            title: title,
            content: content,
            published_date: Utc::now(),
        };
        entity.validate()?;
        Ok(entity)
    }

    pub fn update(&self, title: String, content: String) -> Result<Self, ApiError> {
        let mut entity = self.clone();
        entity.title = title;
        entity.content = content;
        entity.validate()?;
        Ok(entity)
    }

    pub const TITLE_MAX_LENGTH: usize = 60;
    pub const CONTENT_MAX_LENGTH: usize = 20000;
}
