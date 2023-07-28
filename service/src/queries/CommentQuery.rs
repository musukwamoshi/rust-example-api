use ::entity::{comment, comment::Entity as Comment};
use sea_orm::*;

pub struct CommentQuery;

impl CommentQuery {
    pub async fn get_comment_by_id(db: &DbConn, id: i32) -> Result<Option<comment::Model>, DbErr> {
        Comment::find_by_id(id).one(db).await
    }

    /// If ok, returns (article models, num pages).
    pub async fn find_comments_in_page(
        db: &DbConn,
        page: u64,
        comments_per_page: u64,
    ) -> Result<(Vec<comment::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Comment::find()
            .order_by_asc(comment::Column::Id)
            .paginate(db, comments_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated comment
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
