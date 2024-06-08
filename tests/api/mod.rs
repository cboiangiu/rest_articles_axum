pub mod domain;
pub mod features;
use mockall::mock;
use rest_articles_axum::framework::infrastructure::persistence::mongo_repository::{
    MongoRepository, WithMongoRepository,
};
use rest_articles_axum::modules::api::domain::articles::article::Article;
use rest_articles_axum::modules::api::persistence::ArticleRepository;

mock! {
    ArticleRepository {}     // Name of the mock struct, less the "Mock" prefix
    impl ArticleRepository for ArticleRepository {   // specification of the trait to mock
    }
    impl WithMongoRepository<Article> for ArticleRepository {   // specification of the trait to mock
        fn base(&self) -> Box<dyn MongoRepository<Article>>;
    }
}
