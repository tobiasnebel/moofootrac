use chrono::{DateTime, Utc};
use models::models::MooFooLogGetDto;
use models::models::MooFooLogPostDto;
use sea_orm::{ActiveValue::NotSet, Set};

use crate::api::handlers::WithResolvedUserName;
use crate::persistence::entities::moofoolog;

impl TryFrom<WithResolvedUserName<MooFooLogPostDto>> for moofoolog::ActiveModel {
    type Error = String;

    fn try_from(value: WithResolvedUserName<MooFooLogPostDto>) -> Result<Self, Self::Error> {
        Ok(moofoolog::ActiveModel {
            id: NotSet,
            timestamp: Set(Utc::now()),
            user_name: Set(value.user_name),
            mood: Set(value.data.mood),
            food1: Set(value.data.food1.unwrap_or("N/A".to_string())),
            food1_time: Set(value.data.food1_time.unwrap_or("N/A".to_string())),
            food2: Set(value.data.food2.unwrap_or("N/A".to_string())),
            food2_time: Set(value.data.food2_time.unwrap_or("N/A".to_string())),
        })
    }
}

impl From<&moofoolog::Model> for MooFooLogGetDto {
    fn from(value: &moofoolog::Model) -> Self {
        MooFooLogGetDto {
            timestamp: value.timestamp.to_rfc3339(),
            user_name: value.user_name.to_owned(),
            mood: value.mood.to_owned(),
            food1: Some(value.food1.to_owned()),
            food1_time: Some(value.food1_time.to_owned()),
            food2: Some(value.food2.to_owned()),
            food2_time: Some(value.food2_time.to_owned()),
        }
    }
}

// === UTILS ===

#[allow(unused)]
pub fn parse_timestamp_string_to_date_time_utc(
    timestamp_str: String,
) -> Result<chrono::DateTime<Utc>, String> {
    let parsed_date_timestamp = timestamp_str
        .parse::<i64>()
        .map_err(|e| format!("'timestamp' is not a number: {e}"))?;

    let date = DateTime::from_timestamp_millis(parsed_date_timestamp)
        .ok_or_else(|| format!("'timestamp' out of range"))?;

    Ok(date)
}
