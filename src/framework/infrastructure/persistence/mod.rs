pub mod fake_mongo_repository;
pub mod mongo_repository;
use crate::framework::core::domain::EntityWithId;

#[derive(Debug, Clone, Default)]
pub struct PaginatedResult<T: EntityWithId> {
    pub data: Vec<T>,
    pub total: u64,
}
