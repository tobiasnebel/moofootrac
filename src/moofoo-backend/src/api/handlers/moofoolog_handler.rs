use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use models::models::MooFooLogDto;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    api::{errors::CustomError, router::AppState}, persistence::entities::moofoolog,
};

#[derive(Deserialize)]
pub struct RoomQueryParams {
    #[serde(rename = "roomId")]
    room_id: i64,
    page: Option<u64>,
    #[serde(rename = "pageSize")]
    page_size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MooFooLogPage {
    page: u64,
    page_size: u64,
    data: Vec<MooFooLogDto>,
}

pub async fn get_moofoologs(
    Query(q): Query<RoomQueryParams>,
    state: State<AppState>,
) -> Result<(StatusCode, Json<MooFooLogPage>), CustomError> {
    let page_size = q.page_size.unwrap_or(10);
    let page = q.page.unwrap_or(0);
    let res = moofoolog::Entity::find()
        .into_model::<moofoolog::Model>()
        .paginate(&state.conn, page_size)
        .fetch_page(page)
        .await?
        .iter()
        .map(MooFooLogDto::from)
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

pub async fn post_moofoolog(
    state: State<AppState>,
    Json(event_dto): Json<MooFooLogDto>,
) -> Result<StatusCode, CustomError> {
    // ...
    todo!()
}
