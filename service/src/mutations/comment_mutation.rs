use ::entity::{comment, comment::Entity as Comment};
use sea_orm::*;

pub struct CommentMutation;

impl CommentMutation {
    pub async fn create_comment(
        db: &DbConn,
        data: comment::Model,
    ) -> Result<comment::ActiveModel, DbErr> {
        comment::ActiveModel {
            commenter_name: Set(data.commenter_name.to_owned()),
            comment_content: Set(data.comment_content.to_owned()),
            article_id: Set(data.article_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_comment_by_id(
        db: &DbConn,
        id: i32,
        data: comment::Model,
    ) -> Result<comment::Model, DbErr> {
        let comment: comment::ActiveModel = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find comment.".to_owned()))
            .map(Into::into)?;

        comment::ActiveModel {
            id: comment.id,
            commenter_name: Set(data.commenter_name.to_owned()),
            comment_content: Set(data.comment_content.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_comment(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let comment: comment::ActiveModel = Comment::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find comment.".to_owned()))
            .map(Into::into)?;

        comment.delete(db).await
    }

    pub async fn delete_all_comments(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Comment::delete_many().exec(db).await
    }
}
