use ::entity::{reply, reply::Entity as Reply};
use sea_orm::*;

pub struct ReplyQuery;

impl ReplyQuery {
    pub async fn get_replies_by_id(db: &DbConn, id: i32) -> Result<Option<reply::Model>, DbErr> {
        Reply::find_by_id(id).one(db).await
    }

    /// If ok, returns (reply models, num pages).
    pub async fn find_replies_in_page(
        db: &DbConn,
        page: u64,
        replies_per_page: u64,
    ) -> Result<(Vec<reply::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Reply::find()
            .order_by_asc(reply::Column::Id)
            .paginate(db, replies_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated replies
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
