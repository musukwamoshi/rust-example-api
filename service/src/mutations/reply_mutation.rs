use ::entity::{reply, reply::Entity as Reply};
use sea_orm::*;

pub struct ReplyMutation;

impl ReplyMutation {
    pub async fn create_reply(
        db: &DbConn,
        data: reply::Model,
    ) -> Result<reply::ActiveModel, DbErr> {
        reply::ActiveModel {
            responder_name: Set(data.responder_name.to_owned()),
            reply_content: Set(data.reply_content.to_owned()),
            comment_id: Set(data.comment_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_reply_by_id(
        db: &DbConn,
        id: i32,
        data: reply::Model,
    ) -> Result<reply::Model, DbErr> {
        let reply: reply::ActiveModel = Reply::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find reply.".to_owned()))
            .map(Into::into)?;

        reply::ActiveModel {
            id: reply.id,
            responder_name: Set(data.responder_name.to_owned()),
            reply_content: Set(data.reply_content.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_reply(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let reply: reply::ActiveModel = Reply::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find reply.".to_owned()))
            .map(Into::into)?;

        reply.delete(db).await
    }

    pub async fn delete_all_replies(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Reply::delete_many().exec(db).await
    }
}
