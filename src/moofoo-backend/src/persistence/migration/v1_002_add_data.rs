use chrono::Utc;
use sea_orm::{ActiveValue::NotSet, EntityTrait, Set};
use sea_orm_migration::prelude::*;

use crate::persistence::entities::{access, moofoolog_user};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // ################
        // ### User(s) ###
        // ################
        moofoolog_user::Entity::insert(moofoolog_user::ActiveModel {
            id: NotSet,
            user_id: Set("debug_user".to_string()),
            user_name: Set(Some("Dummy Debug user".to_string())),
            password: Set("foobar4223".to_string()),
        })
        .exec(manager.get_connection())
        .await?;

        moofoolog_user::Entity::insert(moofoolog_user::ActiveModel {
            id: NotSet,
            user_id: Set("sven".to_string()),
            user_name: Set(Some("Sveniiii!".to_string())),
            password: Set("foobar4223".to_string()),
        })
        .exec(manager.get_connection())
        .await?;

        // ################
        // ### Token(s) ###
        // ################
        access::Entity::insert(access::ActiveModel {
            id: NotSet,
            created_timestamp: Set(Utc::now()),
            user_id: Set("debug_user".to_string()),
            token: Set("debug_token".to_string()),
        })
        .exec(manager.get_connection())
        .await?;

        access::Entity::insert(access::ActiveModel {
            id: NotSet,
            created_timestamp: Set(Utc::now()),
            user_id: Set("sven".to_string()),
            token: Set("da39a3ee5e6b4b0d3255bfef95601890afd80709".to_string()),
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
