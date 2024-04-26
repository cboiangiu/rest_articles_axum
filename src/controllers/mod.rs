pub mod article_controller;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaginationParams {
    page_number: usize,
    page_size: usize,
}
