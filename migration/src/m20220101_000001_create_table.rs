use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::Hash).string().not_null())
                    .col(ColumnDef::new(User::Salt).string().not_null())
                    .col(ColumnDef::new(User::IsAdmin).boolean().default(false))
                    .col(
                        ColumnDef::new(User::PasswordResetExpiration)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::PasswordResetToken).string().not_null())
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::UserEmail).string().not_null())
                    .col(ColumnDef::new(Session::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-session-user_id")
                            .from(Session::Table, Session::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Article::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::Content).string().not_null())
                    .col(ColumnDef::new(Article::Approved).boolean().default(false))
                    .col(ColumnDef::new(Article::AuthorId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-article-author_id")
                            .from(Article::Table, Article::AuthorId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Comment::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Comment::CommenterName).string().not_null())
                    .col(ColumnDef::new(Comment::CommentContent).string().not_null())
                    .col(ColumnDef::new(Comment::Approved).boolean().default(false))
                    .col(ColumnDef::new(Comment::ArticleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-article_id")
                            .from(Comment::Table, Comment::ArticleId)
                            .to(Article::Table, Article::Id),
                    )
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Reply::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reply::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reply::ResponderName).string().not_null())
                    .col(ColumnDef::new(Reply::ReplyContent).string().not_null())
                    .col(ColumnDef::new(Reply::Approved).boolean().default(false))
                    .col(ColumnDef::new(Reply::CommentId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reply-comment_id")
                            .from(Reply::Table, Reply::CommentId)
                            .to(Comment::Table, Comment::Id),
                    )
                    .to_owned(),
            )
            .await;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await;
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await;
        manager
            .drop_table(Table::drop().table(Reply::Table).to_owned())
            .await;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await;
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Article {
    Table,
    Id,
    Title,
    Content,
    Approved,
    AuthorId,
}

#[derive(Iden)]
enum Comment {
    Table,
    Id,
    CommenterName,
    CommentContent,
    Approved,
    ArticleId,
}

#[derive(Iden)]
enum Reply {
    Table,
    Id,
    ResponderName,
    ReplyContent,
    Approved,
    CommentId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    Hash,
    Salt,
    IsAdmin,
    PasswordResetExpiration,
    PasswordResetToken,
}

#[derive(Iden)]
enum Session {
    Table,
    Id,
    UserId,
    UserEmail,
}
