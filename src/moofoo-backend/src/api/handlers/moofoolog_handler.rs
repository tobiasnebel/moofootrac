use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use models::models::{MooFooLogGetDto, MooFooLogPostDto};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    api::{errors::CustomError, router::AppState},
    metrics::AppMetrics,
    persistence::{
        entities::moofoolog::{self, ActiveModel, Column},
        repositories::get_user_name_from_token,
    },
};

use super::{WithResolvedUserName, TOKEN_HEADER_NAME};

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



/// Checks:
///   - presence of token header
///   - validity of token header
///   - user existence
async fn check_token(
    _op: &str,
    headers: HeaderMap,
    metrics: AppMetrics,
    db: &DatabaseConnection,
) -> Result<super::UserName, CustomError> {
    let token = headers
        .get(TOKEN_HEADER_NAME)
        .ok_or_else(|| CustomError::Unauthorized("Missing token".to_string()))
        // inspect this here to log 'uninformed' access attempts
        .inspect_err(|_e| {
            warn!("[{_op}] Missing token: {_e:?}");
            metrics.missing_token_counter.increment(1)
        })?
        .to_str()
        .map_err(|_e| CustomError::Unauthorized("Invalid token".to_string()))
        // inspect this here to log 'uninformed' access attempts
        .inspect_err(|_e| {
            warn!("[{_op}] Invalid token: {_e:?}");
            metrics.invalid_token_counter.increment(1)
        })?
        .to_string();
    let user_name = get_user_name_from_token(&db, token)
        .await
        // inspect this here to log successful access attempts
        .inspect(|_u| {
            warn!("[{_op}] Valid token of user: {_u:?}");
            metrics.valid_token_counter.increment(1)
        })?;

    Ok(user_name)
}

///
/// GET handler
///
pub async fn get_moofoologs(
    Query(q): Query<LogQueryParams>,
    headers: HeaderMap,
    state: State<AppState>,
) -> Result<(StatusCode, Json<MooFooLogPage>), CustomError> {
    let db = state.0.conn;
    let metrics = state.0.metrics;

    // === "AUTH" ===
    let user_name = check_token("GET", headers, metrics, &db).await?;

    let page_size = q.page_size.unwrap_or(10);
    let page = q.page.unwrap_or(0);
    let res = moofoolog::Entity::find()
        .filter(Column::UserName.eq(user_name)) // only get logs for the user behind our token.
        .into_model::<moofoolog::Model>()
        .paginate(&db, page_size)
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
    let user_name = check_token("POST", headers, metrics, &db).await?;

    let resolved = WithResolvedUserName::with_data_and_user(user_name.to_owned(), log_dto);

    // === DB ===
    // map dto
    let logentry =
        ActiveModel::try_from(resolved).map_err(|e| CustomError::BadRequest(e.to_string()))?;
    // save to db
    logentry.save(&db).await?;

    info!("Sucessfully saved log for user '{user_name}'");

    Ok(StatusCode::CREATED)
}
