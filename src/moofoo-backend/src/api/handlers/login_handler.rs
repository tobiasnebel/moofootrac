use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use tracing::warn;

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::entities::{access, moofoolog_user},
};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Serialize)]
pub struct ResponseData {
    #[serde(rename = "token")]
    token: String,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "userName")]
    user_name: String,
}

pub async fn get_login_handler(
    headers: HeaderMap,
    state: State<AppState>,
) -> Result<(StatusCode, Json<ResponseData>), CustomError> {
    let db = state.0.conn;
    let metrics = state.0.metrics;

    // === "AUTH" ===
    let auth = headers
        .get("Authorization")
        .ok_or_else(|| CustomError::Unauthorized("Missing Authorization".to_string()))
        // inspect this here to log 'uninformed' access attempts
        .inspect_err(|_e| {
            warn!("Missing header: {_e:?}");
            metrics.login_fail_counter.increment(1)
        })?
        .to_str()
        .map_err(|_e| CustomError::Unauthorized("Invalid token".to_string()))
        // inspect this here to log 'uninformed' access attempts
        .inspect_err(|_e| {
            warn!("Invalid header: {_e:?}");
            metrics.login_fail_counter.increment(1)
        })?
        .strip_prefix("Basic ")
        .ok_or(CustomError::BadRequest(
            "Header present, but missing 'Basic ' prefix".to_string(),
        ))?
        .to_string();

    // Decode Base64
    let decoded = STANDARD
        .decode(auth)
        .map_err(|_| CustomError::BadRequest("Invalid base64".to_string()))?;
    let decoded_str = String::from_utf8(decoded)
        .map_err(|_| CustomError::BadRequest("Invalid UTF-8 in decoded data".to_string()))?;

    // Split into username:password
    // FIXME: use password hashes instead of plain text!
    let (user, pass) = decoded_str.split_once(':').ok_or(CustomError::BadRequest(
        "Missing ':' separator in credentials".to_string(),
    ))?;

    // query DB
    let user_db = moofoolog_user::Entity::find()
        .filter(moofoolog_user::Column::UserId.eq(user))
        .one(&db)
        .await?
        .ok_or(CustomError::BadRequest("Unknown user".to_string()))?;

    // check credentials
    if user_db.password.eq(&pass) {
        //
        // FIXME: refresh token here
        //
        let entry = access::Entity::find()
            .filter(access::Column::UserName.eq(&user_db.user_id))
            .one(&db)
            .await?
            .ok_or_else(|| {
                CustomError::InternalServerError(format!(
                    "no token found for user '{}'",
                    user_db.user_id
                ))
            })?;

        Ok((
            StatusCode::OK,
            Json(ResponseData {
                token: entry.token,
                user_id: user_db.user_id,
                user_name: user_db.user_name.unwrap_or_else(|| "<unknown>".to_string()),
            }),
        ))
    } else {
        Err(CustomError::Unauthorized("moo".to_string()))
    }
}
