pub mod article;
use axum::async_trait;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseEntity {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
}

#[async_trait]
pub trait EntityWithId:
    Send + Sync + Debug + Default + Clone + Serialize + for<'a> Deserialize<'a> + Unpin
{
    fn get_id(&self) -> &ObjectId;
    fn set_id(&mut self, id: ObjectId);
}
