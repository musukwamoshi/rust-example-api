use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::Hash).string().null())
                    .modify_column(ColumnDef::new(User::Salt).string().null())
                    .modify_column(
                        ColumnDef::new(User::PasswordResetExpiration)
                            .string()
                            .null(),
                    )
                    .modify_column(ColumnDef::new(User::PasswordResetToken).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .modify_column(ColumnDef::new(User::Hash).string().not_null())
                    .modify_column(ColumnDef::new(User::Salt).string().not_null())
                    .modify_column(
                        ColumnDef::new(User::PasswordResetExpiration)
                            .string()
                            .not_null(),
                    )
                    .modify_column(ColumnDef::new(User::PasswordResetToken).string().not_null())
                    .to_owned(),
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Hash,
    Salt,
    PasswordResetExpiration,
    PasswordResetToken,
}
