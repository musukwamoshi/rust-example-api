use ::entity::{user, user::Entity as User};
use sea_orm::*;

pub struct UserMutation;

impl UserMutation {
    pub async fn create_user(db: &DbConn, data: user::Model) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            email: Set(data.email.to_owned()),
            first_name: Set(data.first_name.to_owned()),
            last_name: Set(data.last_name.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
            id: user.id,
            email: Set(data.email.to_owned()),
            first_name: Set(data.first_name.to_owned()),
            last_name: Set(data.last_name.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }
}
