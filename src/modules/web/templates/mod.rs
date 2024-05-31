use crate::modules::api::features::articles::article::get_article_list_v1::GetArticleListResponse;
use o2o::o2o;

#[derive(askama::Template)]
#[template(path = "web/index.html")]
pub struct IndexPage {
    pub article_list_component: ArticleListComponent,
}

#[derive(o2o, askama::Template)]
#[from_owned(GetArticleListResponse)]
#[template(path = "web/components/articles/article.html")]
pub struct ArticleComponent {
    pub id: String,
    pub title: String,
    pub content: String,
    #[from(~.to_string())]
    pub published_date: String,
}

#[derive(askama::Template)]
#[template(path = "web/components/articles/article-list.html")]
pub struct ArticleListComponent {
    pub article_list: Vec<ArticleComponent>,
}
