use axum::{
    Json,
    extract::{Path, Query, State},
    http,
    http::{HeaderMap, StatusCode},
};
use chrono::Utc;
use models::models::{MooFooLogGetDto, MooFooLogPostDto};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    api::{errors::CustomError, router::AppState},
    metrics::AppMetrics,
    persistence::{
        entities::moofoolog::{self, ActiveModel, Column},
        repositories::get_user_id_from_token,
    },
};

use super::{TOKEN_HEADER_NAME, WithResolvedUserId};

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
) -> Result<super::UserId, CustomError> {
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
    let user_name = get_user_id_from_token(&db, token)
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
    let user_id = check_token("GET", headers, metrics, &db).await?;

    let page_size = q.page_size.unwrap_or(10);
    let page = q.page.unwrap_or(0);
    let res = moofoolog::Entity::find()
        .filter(Column::UserId.eq(user_id)) // only get logs for the user behind our token.
        .order_by_desc(Column::Timestamp)
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

    let resolved = WithResolvedUserId::with_data_and_user(user_name.to_owned(), log_dto);

    // === DB ===
    // map dto
    let logentry =
        ActiveModel::try_from(resolved).map_err(|e| CustomError::BadRequest(e.to_string()))?;
    // save to db
    logentry.save(&db).await?;

    info!("Sucessfully saved log for user '{user_name}'");

    Ok(StatusCode::CREATED)
}

///
/// DELETE handler
///
pub async fn delete_moofoolog(
    Path(id): Path<String>,
    headers: HeaderMap,
    state: State<AppState>,
) -> Result<StatusCode, CustomError> {
    let db = state.0.conn;
    let metrics = state.0.metrics;

    // === "AUTH" ===
    let user_id = check_token("DELETE", headers, metrics, &db).await?;

    let id_parsed: i64 = id
        .parse()
        .map_err(|_e| CustomError::BadRequest("Illegal id".to_string()))?;

    let res = moofoolog::Entity::delete_many()
        .filter(moofoolog::Column::Id.eq(id_parsed))
        .filter(moofoolog::Column::UserId.eq(user_id))
        .exec(&db)
        .await?;

    assert!(
        res.rows_affected < 2,
        "whenever more than one entry has been deleted here, our where/filter contidion is faulty!"
    );

    warn!("Deleted {} entrie(s)", res.rows_affected);

    Ok(StatusCode::OK)
}

///
/// GET handler for excel export
///
pub async fn get_moofoologs_export(
    headers: HeaderMap,
    state: State<AppState>,
) -> Result<([(http::HeaderName, http::HeaderValue); 2], Vec<u8>), CustomError> {
    let db = state.0.conn;
    let metrics = state.0.metrics;

    // === "AUTH" ===
    let user_id = check_token("EXPORT", headers, metrics, &db).await?;

    let count = moofoolog::Entity::find()
        .filter(Column::UserId.eq(&user_id)) // only get logs for the user behind our token.
        .order_by_desc(Column::Timestamp)
        .into_model::<moofoolog::Model>()
        .count(&db)
        .await?;
    if count > 5000 {
        return Err(CustomError::BadRequest(
            "Too many entries to export. Maximum is 5000.".to_string(),
        ));
    }

    let res = moofoolog::Entity::find()
        .filter(Column::UserId.eq(&user_id)) // only get logs for the user behind our token.
        .order_by_desc(Column::Timestamp)
        .into_model::<moofoolog::Model>()
        .all(&db)
        .await?;

    use rust_xlsxwriter::{Workbook, XlsxError};
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add headers
    worksheet.write_string(0, 0, "Id")?;
    worksheet.write_string(0, 1, "Timestamp")?;
    worksheet.write_string(0, 2, "Mood")?;
    worksheet.write_string(0, 3, "Food1")?;
    worksheet.write_string(0, 4, "Food1Time")?;
    worksheet.write_string(0, 5, "Food2")?;
    worksheet.write_string(0, 6, "Food2Time")?;

    // Write data
    for (index, row) in res.iter().enumerate() {
        let row_num = (index + 1) as u32;
        worksheet.write_number(row_num, 0, row.id as f64)?;
        worksheet.write_string(row_num, 1, &row.timestamp.to_string())?;
        worksheet.write_string(row_num, 2, &row.mood)?;
        if !row.food1.is_empty() {
            worksheet.write_string(row_num, 3, &row.food1)?;
        }
        if !row.food1_time.is_empty() {
            worksheet.write_string(row_num, 4, &row.food1_time.to_string())?;
        }
        if !row.food2.is_empty() {
            worksheet.write_string(row_num, 5, &row.food2)?;
        }
        if !row.food2_time.is_empty() {
            worksheet.write_string(row_num, 6, &row.food2_time.to_string())?;
        }
    }

    let buffer = workbook.save_to_buffer().map_err(|e: XlsxError| {
        CustomError::InternalServerError(format!("Failed to create Excel file: {}", e))
    })?;

    let now: String = Utc::now().format("%Y-%m-%dT%H-%M-%S").to_string();
    let headers = [
        (
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static(
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            ),
        ),
        (
            http::header::CONTENT_DISPOSITION,
            http::HeaderValue::from_str(
                format!("attachment; filename=\"moofoolog_export_{now}.xlsx\"").as_str(),
            )?,
        ),
    ];

    Ok((headers, buffer))
}
