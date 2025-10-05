use sea_orm::entity::prelude::*;
use sea_orm_migration::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "access")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub created_timestamp: DateTimeUtc,
    pub user_name: String,
    pub token: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(DeriveIden)]
pub enum Access {
    Table,
    Id,
    CreatedTimestamp,
    UserName,
    Token,
}
