pub mod repositories;
use crate::entities::article::Article;
use mongodb::{options::ClientOptions, Client, Collection};

pub struct Collections {
    pub articles: Collection<Article>,
}

pub async fn connect_to_mongo() -> Result<Collections, mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27003").await?;
    let client = Client::with_options(client_options)?;

    let database = client.database("axum_articles_db");
    let articles: Collection<Article> = database.collection::<Article>("articles");
    let collections = Collections { articles };

    Ok(collections)
}
