pub mod domain;
pub mod errors;
pub mod paging;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateEntitiesResponse {
    pub entities_created: usize,
    pub entities_updated: usize,
}
