use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use models::models::{MooFooLogGetDto, MooFooLogPostDto};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{
    api::{errors::CustomError, router::AppState},
    persistence::{
        entities::moofoolog::{self, ActiveModel, Column},
        repositories::get_user_name_from_token,
    },
};

use super::WithResolvedUserName;

#[derive(Deserialize)]
pub struct LogQueryParams {
    page: Option<u64>,
    #[serde(rename = "pageSize")]
    page_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooFooLogPage {
    page: u64,
    page_size: u64,
    data: Vec<MooFooLogGetDto>,
}

/// that header name, yikes
pub const TOKEN_HEADER_NAME: &str = "MooFoo-Token";

///
/// GET handler
///
pub async fn get_moofoologs(
    Query(q): Query<LogQueryParams>,
    headers: HeaderMap,
    state: State<AppState>,
) -> Result<(StatusCode, Json<MooFooLogPage>), CustomError> {
    let token = headers
        .get(TOKEN_HEADER_NAME)
        .ok_or(CustomError::Unauthorized("Missing token".to_string()))?
        .to_str()
        .map_err(|_e| CustomError::Unauthorized("Invalid token".to_string()))?
        .to_string();
    let user_name = get_user_name_from_token(&state.conn, token).await?;

    let page_size = q.page_size.unwrap_or(10);
    let page = q.page.unwrap_or(0);
    let res = moofoolog::Entity::find()
        .filter(Column::UserName.eq(user_name)) // only get logs for the user behind our token.
        .into_model::<moofoolog::Model>()
        .paginate(&state.conn, page_size)
        .fetch_page(page)
        .await?
        .iter()
        .map(MooFooLogGetDto::from)
        .collect();
    Ok((
        StatusCode::OK,
        Json(MooFooLogPage {
            page,
            page_size,
            data: res,
        }),
    ))
}

///
/// POST handler
///
pub async fn post_moofoolog(
    headers: HeaderMap,
    state: State<AppState>,
    Json(log_dto): Json<MooFooLogPostDto>,
) -> Result<StatusCode, CustomError> {
    let db = state.0.conn;
    let metrics = state.0.metrics;

    // === "AUTH" ===
    let token = headers
        .get(TOKEN_HEADER_NAME)
        .ok_or(CustomError::Unauthorized("Missing token".to_string()))
        // inspect this here to log 'uninformed' access attempts
        .inspect_err(|_e| {
            warn!("Missing token: {_e:?}");
            metrics.missing_token_counter.increment(1)
        })?
        .to_str()
        .map_err(|_e| CustomError::Unauthorized("Invalid token".to_string()))
        // inspect this here to log 'informed' access attempts
        .inspect_err(|_e| {
            warn!("Invalid token: {_e:?}");
            metrics.invalid_token_counter.increment(1)
        })?
        .to_string();

    let user_name = get_user_name_from_token(&db, token)
        .await
        // inspect this here to log successful access attempts
        .inspect(|_u| {
            warn!("Successful login of user: {_u:?}");
            metrics.successful_login_counter.increment(1)
        })?;

    let resolved = WithResolvedUserName::with_data_and_user(user_name, log_dto);

    // === DB ===
    // map dto
    let logentry =
        ActiveModel::try_from(resolved).map_err(|e| CustomError::BadRequest(e.to_string()))?;
    // save to db
    logentry.save(&db).await?;

    Ok(StatusCode::CREATED)
}
