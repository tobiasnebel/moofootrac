use sea_orm::entity::prelude::*;
use sea_orm_migration::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "moofoolog_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub user_id: String,
    pub user_name: Option<String>,
    pub password: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(DeriveIden)]
pub enum MooFooLogUser {
    #[sea_orm(iden = "moofoolog_user")]
    Table,
    Id,
    UserId,
    UserName,
    Password,
}
