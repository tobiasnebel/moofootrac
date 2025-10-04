use chrono::{DateTime, Utc};
use models::models::MooFooLogDto;
use sea_orm::{ActiveValue::NotSet, Set};

use crate::persistence::entities::moofoolog;

impl TryFrom<MooFooLogDto> for moofoolog::ActiveModel {
    type Error = String;

    fn try_from(value: MooFooLogDto) -> Result<Self, Self::Error> {
        Ok(moofoolog::ActiveModel {
            id: NotSet,
            timestamp: Set(parse_timestamp_string_to_date_time_utc(value.timestamp)?),
            user: Set(value.user_name),
            mood: Set(value.mood),
            food1: Set(value.food1.unwrap_or("N/A".to_string())),
            food1_time: Set(value.food1_time.unwrap_or("N/A".to_string())),
            food2: Set(value.food2.unwrap_or("N/A".to_string())),
            food2_time: Set(value.food2_time.unwrap_or("N/A".to_string())),
        })
    }
}

impl From<&moofoolog::Model> for MooFooLogDto {
    fn from(value: &moofoolog::Model) -> Self {
        MooFooLogDto {
            timestamp: value.timestamp.to_rfc3339(),
            user_name: "THE_USER".to_string(),
            mood: value.mood.to_owned(),
            food1: Some(value.food1.to_owned()),
            food1_time: Some(value.food1_time.to_owned()),
            food2: Some(value.food2.to_owned()),
            food2_time: Some(value.food2_time.to_owned()),
        }
    }
}

// === UTILS ===

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
