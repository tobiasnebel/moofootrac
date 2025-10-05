use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::entities::access,
};

#[derive(Deserialize)]
pub struct QueryParams {
    #[serde(rename = "auth")]
    auth: Option<String>,
}

#[derive(Serialize)]
pub struct ResponseData {
    #[serde(rename = "token")]
    token: String,
}

pub async fn get_login_handler(
    Query(q): Query<QueryParams>,
    _state: State<AppState>,
) -> Result<(StatusCode, Json<ResponseData>), CustomError> {
    // FIXME: retrieve debug_token with actual credentials
    if let Some(_auth) = q.auth
        && _auth.eq(&"foobar4223".to_string())
    {
        let entry = access::Entity::find()
            .filter(access::Column::UserName.eq("debug_user"))
            .one(&_state.0.conn)
            .await?
            .ok_or_else(|| {
                CustomError::InternalServerError(
                    "no token found for dummy 'debug_user'".to_string(),
                )
            })?;

        Ok((StatusCode::OK, Json(ResponseData { token: entry.token })))
    } else {
        Err(CustomError::Unauthorized("moo".to_string()))
    }
}
