use ::entity::{article, article::Entity as Article};
use sea_orm::*;

pub struct ArticleMutation;

impl ArticleMutation {
    pub async fn create_article(
        db: &DbConn,
        data: article::Model,
    ) -> Result<article::ActiveModel, DbErr> {
        article::ActiveModel {
            title: Set(data.title.to_owned()),
            content: Set(data.content.to_owned()),
            author_id: Set(data.author_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_article_by_id(
        db: &DbConn,
        id: i32,
        data: article::Model,
    ) -> Result<article::Model, DbErr> {
        let article: article::ActiveModel = Article::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find article.".to_owned()))
            .map(Into::into)?;

        article::ActiveModel {
            id: article.id,
            title: Set(data.title.to_owned()),
            content: Set(data.content.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn delete_article(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let article: article::ActiveModel = Article::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find article.".to_owned()))
            .map(Into::into)?;

        article.delete(db).await
    }

    pub async fn delete_all_articles(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Article::delete_many().exec(db).await
    }
}
