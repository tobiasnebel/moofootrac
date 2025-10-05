use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::api::errors::CustomError;

use super::entities::access;

pub type UserName = String;

pub async fn get_user_name_from_token(
    db_conn: &DatabaseConnection,
    token: UserName,
) -> Result<String, CustomError> {
    let db_res = access::Entity::find()
        .filter(access::Column::Token.eq(token))
        .one(db_conn)
        .await?;

    let res = match db_res {
        Some(model) => Ok(model.user_name),
        None => Err(CustomError::Unauthorized("Invalid token".to_string())),
    };

    res
}
