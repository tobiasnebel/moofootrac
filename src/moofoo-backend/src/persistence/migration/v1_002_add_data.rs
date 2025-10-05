use chrono::Utc;
use sea_orm::{ActiveValue::NotSet, EntityTrait, Set};
use sea_orm_migration::prelude::*;

use crate::persistence::entities::access;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ################
        // ### Token(s) ###
        // ################
        access::Entity::insert(access::ActiveModel {
            id: NotSet,
            created_timestamp: Set(Utc::now()),
            user_name: Set("debug_user".to_string()),
            token: Set("debug_token".to_string()),
        })
        .exec(manager.get_connection())
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // clean all inserted rooms
        access::Entity::delete_many()
            .exec(manager.get_connection())
            .await
            .map(|_delres| ())
    }
}
