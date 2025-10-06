use sea_orm_migration::prelude::*;

use crate::persistence::entities::{
    access::Access, moofoolog::MooFooLog, moofoolog_user::MooFooLogUser,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // create 'moofoolog' table
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(MooFooLog::Table)
                    .col(
                        ColumnDef::new(MooFooLog::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MooFooLog::Timestamp)
                            .default(Expr::current_timestamp())
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MooFooLog::UserName).string().not_null())
                    .col(ColumnDef::new(MooFooLog::Mood).string().not_null())
                    .col(ColumnDef::new(MooFooLog::Food1).string().null())
                    .col(ColumnDef::new(MooFooLog::Food1Time).string().null())
                    .col(ColumnDef::new(MooFooLog::Food2).string().null())
                    .col(ColumnDef::new(MooFooLog::Food2Time).string().null())
                    .to_owned(),
            )
            .await?;

        // create 'moofoolog_user' table
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(MooFooLogUser::Table)
                    .col(
                        ColumnDef::new(MooFooLogUser::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(MooFooLogUser::UserId)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MooFooLogUser::UserName).string())
                    .col(ColumnDef::new(MooFooLogUser::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        // create 'access' table
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Access::Table)
                    .col(
                        ColumnDef::new(Access::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Access::CreatedTimestamp)
                            .default(Expr::current_timestamp())
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Access::UserName)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Access::Token).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MooFooLog::Table).to_owned())
            .await
    }
}
