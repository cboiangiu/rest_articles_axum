#[cfg(test)]
use crate::api::MockArticleRepository;
use bson::oid::ObjectId;
use rest_articles_axum::{
    framework::core::domain::EntityWithId,
    modules::api::{
        domain::articles::article::Article, features::articles::article::create_update_articles_v1,
    },
};
use std::vec;
use tokio::test;

fn setup() -> Vec<Article> {
    let mut articles = vec![];
    for i in 1..=15 {
        let title = format!("Test Title {}", i);
        let content = format!("Test Content {}", i);
        let mut article = Article::create(title, content).unwrap();
        article.set_id(ObjectId::new());
        articles.push(article);
    }
    articles
}

#[allow(unused_variables)]
#[test]
async fn handle_success() {
    let articles = setup();
    let command = create_update_articles_v1::CreateUpdateArticlesCommand {
        articles: || -> Vec<create_update_articles_v1::CreateUpdateArticleCommand> {
            let mut commands: Vec<create_update_articles_v1::CreateUpdateArticleCommand> = vec![];
            for article in articles.iter() {
                let command = create_update_articles_v1::CreateUpdateArticleCommand {
                    id: match &article.base.id {
                        Some(id) => Some(id.to_hex()),
                        _ => None,
                    },
                    title: article.title.clone(),
                    content: article.content.clone(),
                };
                commands.push(command);
            }
            commands
        }(),
    };

    let mock_repository = MockArticleRepository::new();

    // mockRepository
    //     .expect_base()
    //     .with(eq(4))
    //     .times(1)
    //     .returning(|x| x + 1);
    // let result = get_article_v1::handle(mockRepository, query);

    // mock.expect_foo().with(eq(4)).times(1).returning(|x| x + 1);
    // assert_eq!(5, mock.foo(4));
    // let (_, _, article_service) = setup(None);
    // let create_dto = CreateArticleDTO {
    //     title: "Test Title".to_string(),
    //     content: "Test Content".to_string(),
    // };
    // let result = article_service.insert_article(create_dto).await;
    // assert!(result.is_ok());
    // let result = result.unwrap();
    // assert!(result.title == "Test Title");
    // assert!(result.content == "Test Content");
    // assert!(!result.id.is_empty());
    // TODO: assert published_date
}

// #[test]
// async fn get_article_by_id_not_found() {
//     let (_, _, article_service) = setup(None);
//     let result = article_service
//         .get_article_by_id(ObjectId::new().to_hex())
//         .await;
//     assert!(matches!(result, Err(ArticleError::NotFoundError)));
// }

// #[test]
// async fn update_article_success() {
//     let test_article_id = ObjectId::new();
//     let articles = Arc::new(StdMutex::new(vec![Article {
//         base: BaseEntity {
//             id: Some(test_article_id),
//         },
//         title: "Original Title".to_string(),
//         content: "Original Content".to_string(),
//         published_date: chrono::Utc::now(),
//     }]));
//     let (_, _, article_service) = setup(Some(articles.clone()));
//     let update_dto = UpdateArticleDTO {
//         title: "Updated Title".to_string(),
//         content: "Updated Content".to_string(),
//     };
//     let result = article_service
//         .update_article(test_article_id.to_hex(), update_dto)
//         .await;
//     assert!(result.is_ok());
//     let result = result.unwrap();
//     assert!(result.title == "Updated Title");
//     assert!(result.content == "Updated Content");
//     assert!(result.id == test_article_id.to_hex());
// }

// #[test]
// async fn delete_article_not_found() {
//     let test_article_id = ObjectId::new();
//     let articles = Arc::new(StdMutex::new(vec![Article {
//         base: BaseEntity {
//             id: Some(test_article_id),
//         },
//         title: "Test Title".to_string(),
//         content: "Test Content".to_string(),
//         published_date: chrono::Utc::now(),
//     }]));
//     let (_, _, article_service) = setup(Some(articles.clone()));
//     let result = article_service
//         .delete_article(ObjectId::new().to_hex())
//         .await;
//     assert!(matches!(result, Err(ArticleError::NotFoundError)));
//     assert!(articles.lock().unwrap().len() == 1);
// }
