use ::entity::{article, article::Entity as Article};
use sea_orm::*;

pub struct ArticleQuery;

impl ArticleQuery {
    pub async fn get_article_by_id(db: &DbConn, id: i32) -> Result<Option<article::Model>, DbErr> {
        Article::find_by_id(id).one(db).await
    }

    /// If ok, returns (article models, num pages).
    pub async fn find_articles_in_page(
        db: &DbConn,
        page: u64,
        articles_per_page: u64,
    ) -> Result<(Vec<article::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Article::find()
            .order_by_asc(article::Column::Id)
            .paginate(db, articles_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated comment
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
