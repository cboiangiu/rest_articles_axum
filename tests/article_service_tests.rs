#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;
    use rest_articles_axum::data::repositories::fake::fake_article_repository::FakeArticleRepository;
    use rest_articles_axum::dtos::article_dto::{CreateArticleDTO, UpdateArticleDTO};
    use rest_articles_axum::entities::article::Article;
    use rest_articles_axum::entities::BaseEntity;
    use rest_articles_axum::errors::ArticleError;
    use rest_articles_axum::services::article_service::{ArticleService, ArticleServiceImpl};
    use std::sync::{Arc, Mutex as StdMutex};
    use tokio::test;

    fn setup(
        articles: Option<Arc<StdMutex<Vec<Article>>>>,
    ) -> (
        Arc<StdMutex<Vec<Article>>>,
        Arc<FakeArticleRepository>,
        ArticleServiceImpl,
    ) {
        let articles = match articles {
            Some(articles) => articles.clone(),
            None => Arc::new(StdMutex::new(vec![])),
        };
        let mock_repo = Arc::new(FakeArticleRepository::new(articles.clone()));
        let article_service = ArticleServiceImpl::new(mock_repo.clone());
        (articles, mock_repo, article_service)
    }

    #[test]
    async fn insert_article_success() {
        let (_, _, article_service) = setup(None);
        let create_dto = CreateArticleDTO {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };
        let result = article_service.insert_article(create_dto).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.title == "Test Title");
        assert!(result.content == "Test Content");
        assert!(!result.id.is_empty());
        // TODO: assert published_date
    }

    #[test]
    async fn get_article_by_id_not_found() {
        let (_, _, article_service) = setup(None);
        let result = article_service
            .get_article_by_id(ObjectId::new().to_hex())
            .await;
        assert!(matches!(result, Err(ArticleError::NotFoundError)));
    }

    #[test]
    async fn update_article_success() {
        let test_article_id = ObjectId::new();
        let articles = Arc::new(StdMutex::new(vec![Article {
            base: BaseEntity {
                id: Some(test_article_id),
            },
            title: "Original Title".to_string(),
            content: "Original Content".to_string(),
            published_date: chrono::Utc::now(),
        }]));
        let (_, _, article_service) = setup(Some(articles.clone()));
        let update_dto = UpdateArticleDTO {
            title: "Updated Title".to_string(),
            content: "Updated Content".to_string(),
        };
        let result = article_service
            .update_article(test_article_id.to_hex(), update_dto)
            .await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.title == "Updated Title");
        assert!(result.content == "Updated Content");
        assert!(result.id == test_article_id.to_hex());
    }

    #[test]
    async fn delete_article_not_found() {
        let test_article_id = ObjectId::new();
        let articles = Arc::new(StdMutex::new(vec![Article {
            base: BaseEntity {
                id: Some(test_article_id),
            },
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            published_date: chrono::Utc::now(),
        }]));
        let (_, _, article_service) = setup(Some(articles.clone()));
        let result = article_service
            .delete_article(ObjectId::new().to_hex())
            .await;
        assert!(matches!(result, Err(ArticleError::NotFoundError)));
        assert!(articles.lock().unwrap().len() == 1);
    }
}
