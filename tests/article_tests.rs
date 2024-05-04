#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;
    use rest_articles_axum::entities::article::Article;
    use rest_articles_axum::entities::BaseEntity;
    use rest_articles_axum::errors::ArticleError;
    use tokio::test;

    #[test]
    async fn set_title_success() {
        let mut article = Article {
            base: BaseEntity {
                id: Some(ObjectId::new()),
            },
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            published_date: chrono::Utc::now(),
        };
        let result = article.set_title("Updated Title".to_string());
        assert!(result.is_ok());
        assert!(article.title == "Updated Title");
    }

    #[test]
    async fn set_title_article_error() {
        let mut article = Article {
            base: BaseEntity {
                id: Some(ObjectId::new()),
            },
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            published_date: chrono::Utc::now(),
        };

        let result = article.set_title("".to_string());
        assert!(matches!(result, Err(ArticleError::ArgumentNullError(_))));
        assert!(article.title != "");

        let text_too_long = "a".to_string().repeat(Article::TITLE_MAX_LENGTH + 1);
        let result = article.set_title(text_too_long.clone());
        assert!(matches!(result, Err(ArticleError::TextTooLongError(_))));
        assert!(article.title != text_too_long);
    }

    #[test]
    async fn set_content_success() {
        let mut article = Article {
            base: BaseEntity {
                id: Some(ObjectId::new()),
            },
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            published_date: chrono::Utc::now(),
        };
        let result = article.set_title("Updated Content".to_string());
        assert!(result.is_ok());
        assert!(article.title == "Updated Content");
    }

    #[test]
    async fn set_content_article_error() {
        let mut article = Article {
            base: BaseEntity {
                id: Some(ObjectId::new()),
            },
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
            published_date: chrono::Utc::now(),
        };

        let result = article.set_content("".to_string());
        assert!(matches!(result, Err(ArticleError::ArgumentNullError(_))));
        assert!(article.content != "");

        let text_too_long = "a".to_string().repeat(Article::CONTENT_MAX_LENGTH + 1);
        let result = article.set_content(text_too_long.clone());
        assert!(matches!(result, Err(ArticleError::TextTooLongError(_))));
        assert!(article.content != text_too_long);
    }
}
