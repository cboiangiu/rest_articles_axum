use crate::framework::core::errors::ErrorResponse;
use crate::framework::core::paging::PaginatedGetArticleListResponse;
use crate::framework::core::CreateUpdateEntitiesResponse;
use crate::modules::api::features::articles::article::create_update_articles_v1::{
    CreateUpdateArticleCommand, CreateUpdateArticlesCommand, CreateUpdateArticlesResponse,
    __path_create_update_articles_v1_endpoint,
};
use crate::modules::api::features::articles::article::delete_articles_v1::{
    DeleteArticlesCommand, __path_delete_articles_v1_endpoint,
};
use crate::modules::api::features::articles::article::get_article_list_v1::{
    GetArticleListResponse, __path_get_article_list_v1_endpoint,
};
use crate::modules::api::features::articles::article::get_article_v1::{
    GetArticleResponse, __path_get_article_v1_endpoint,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_update_articles_v1_endpoint,
        get_article_v1_endpoint,
        get_article_list_v1_endpoint,
        delete_articles_v1_endpoint,
    ),
    components(schemas(
        CreateUpdateArticlesCommand,
        CreateUpdateArticleCommand,
        CreateUpdateArticlesResponse,
        GetArticleResponse,
        PaginatedGetArticleListResponse,
        GetArticleListResponse,
        DeleteArticlesCommand,
        CreateUpdateEntitiesResponse,
        ErrorResponse
    ))
)]
pub struct ApiDoc;
