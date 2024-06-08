#[cfg(test)]
use rest_articles_axum::{
    framework::core::errors::ApiError, modules::api::domain::articles::article::Article,
};
use tokio::test;

fn setup() -> Article {
    Article::create("Test Title".to_string(), "Test Content".to_string()).unwrap()
}

#[test]
async fn create_article_success() {
    let article = setup();
    let result: Result<Article, rest_articles_axum::framework::core::errors::ApiError> =
        Article::create(article.title.clone(), article.content.clone());
    assert!(result.is_ok());
    assert!(result.as_ref().unwrap().title == article.title);
    assert!(result.as_ref().unwrap().content == article.content);
}

#[test]
async fn update_article_success() {
    let article = setup();
    let result = article
        .clone()
        .update("Updated Title".to_string(), "Updated Content".to_string());
    assert!(result.is_ok());
    assert!(result.as_ref().unwrap().title == "Updated Title".to_string());
    assert!(result.as_ref().unwrap().content == "Updated Content".to_string());
}

#[test]
async fn create_article_error() {
    let article = setup();

    let result = Article::create("".to_string(), article.content.clone());
    assert!(matches!(result, Err(ApiError::ArgumentNullError(_))));
    let result = Article::create(article.title.clone(), "".to_string());
    assert!(matches!(result, Err(ApiError::ArgumentNullError(_))));

    let text_too_long = "a".to_string().repeat(Article::TITLE_MAX_LENGTH + 1);
    let result = Article::create(text_too_long.clone(), article.content.clone());
    assert!(matches!(result, Err(ApiError::TextTooLongError(_))));
    let text_too_long = "a".to_string().repeat(Article::CONTENT_MAX_LENGTH + 1);
    let result = Article::create(article.title.clone(), text_too_long.clone());
    assert!(matches!(result, Err(ApiError::TextTooLongError(_))));
}

#[test]
async fn update_article_error() {
    let article = setup();

    let result = article
        .clone()
        .update("".to_string(), article.content.clone());
    assert!(matches!(result, Err(ApiError::ArgumentNullError(_))));
    let result = article
        .clone()
        .update(article.title.clone(), "".to_string());
    assert!(matches!(result, Err(ApiError::ArgumentNullError(_))));

    let text_too_long = "a".to_string().repeat(Article::TITLE_MAX_LENGTH + 1);
    let result = article
        .clone()
        .update(text_too_long.clone(), article.content.clone());
    assert!(matches!(result, Err(ApiError::TextTooLongError(_))));
    let text_too_long = "a".to_string().repeat(Article::CONTENT_MAX_LENGTH + 1);
    let result = article
        .clone()
        .update(article.title.clone(), text_too_long.clone());
    assert!(matches!(result, Err(ApiError::TextTooLongError(_))));
}
