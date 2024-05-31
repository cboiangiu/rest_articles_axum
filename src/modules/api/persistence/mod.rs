use crate::{
    domain::articles::article::Article,
    framework::infrastructure::persistence::{
        fake_mongo_repository::FakeMongoEntityRepository,
        mongo_repository::{MongoEntityRepository, WithMongoRepository},
    },
};
use axum::async_trait;
use mongodb::{options::ClientOptions, Client, Collection};

pub struct Collections {
    pub articles: Collection<Article>,
}

pub async fn connect_to_mongo() -> Result<Collections, mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("axum_articles_db");
    let articles: Collection<Article> = database.collection::<Article>("articles");
    let collections = Collections { articles };

    Ok(collections)
}

#[async_trait]
pub trait ArticleRepository: WithMongoRepository<Article> + Send + Sync {}

impl ArticleRepository for MongoEntityRepository<Article> {}

impl ArticleRepository for FakeMongoEntityRepository<Article> {}
