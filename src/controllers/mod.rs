pub mod article_controller;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
struct PaginationParams {
    #[param(default = 1)]
    page_number: usize,
    #[param(default = 10)]
    page_size: usize,
}
