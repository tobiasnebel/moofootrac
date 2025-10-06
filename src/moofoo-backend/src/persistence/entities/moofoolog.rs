use sea_orm::entity::prelude::*;
use sea_orm_migration::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "moofoolog")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub timestamp: DateTimeUtc,
    pub user_id: String,
    pub mood: String,
    pub food1: String,
    pub food1_time: String,
    pub food2: String,
    pub food2_time: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(DeriveIden)]
pub enum MooFooLog {
    #[sea_orm(iden = "moofoolog")]
    Table,
    Id,
    Timestamp,
    UserId,
    Mood,
    Food1,
    Food1Time,
    Food2,
    Food2Time,
}
