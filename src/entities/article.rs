use super::{BaseEntity, EntityWithId};
use crate::{dtos::article_dto::ReadArticleDTO, errors::ArticleError};
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
    pub fn new() -> Self {
        Self {
            base: BaseEntity { id: None },
            title: "".to_string(),
            content: "".to_string(),
            published_date: Utc::now(),
        }
    }

    pub fn set_title(&mut self, title: String) -> Result<(), ArticleError> {
        if title.trim().is_empty() {
            return Err(ArticleError::ArgumentNullError("title".to_string()));
        }
        if title.len() > Self::TITLE_MAX_LENGTH {
            return Err(ArticleError::TextTooLongError("title".to_string()));
        }

        self.title = title;
        Ok(())
    }

    pub fn set_content(&mut self, content: String) -> Result<(), ArticleError> {
        if content.trim().is_empty() {
            return Err(ArticleError::ArgumentNullError("content".to_string()));
        }
        if content.len() > Self::CONTENT_MAX_LENGTH {
            return Err(ArticleError::TextTooLongError("content".to_string()));
        }

        self.content = content;
        Ok(())
    }

    pub const TITLE_MAX_LENGTH: usize = 60;
    pub const CONTENT_MAX_LENGTH: usize = 20000;
}

impl Into<ReadArticleDTO> for Article {
    fn into(self) -> ReadArticleDTO {
        ReadArticleDTO {
            id: self.base.id.as_ref().unwrap().to_hex(),
            title: self.title.clone(),
            content: self.content.clone(),
            published_date: self.published_date.clone(),
        }
    }
}
