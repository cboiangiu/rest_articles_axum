pub mod article_entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: i32,
}

pub trait EntityWithId {
    fn get_id(&self) -> i32;
    fn set_id(&mut self, id: i32);
}
