pub mod article_repository;
pub mod fake;
pub mod repository_base;
use crate::entities::EntityWithId;

#[derive(Debug, Clone, Default)]
pub struct PaginatedResult<T: EntityWithId> {
    pub data: Vec<T>,
    pub total: u64,
}
