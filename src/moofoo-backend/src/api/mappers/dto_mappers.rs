use chrono::{DateTime, Utc};
use models::models::MooFooLogDto;
use sea_orm::{ActiveValue::NotSet, Set};

use crate::persistence::entities::moofoolog;

impl TryFrom<MooFooLogDto> for moofoolog::ActiveModel {
    type Error = String;

    fn try_from(value: MooFooLogDto) -> Result<Self, Self::Error> {
        Ok(moofoolog::ActiveModel {
            ..Default::default()
        })
    }
}

impl From<&moofoolog::Model> for MooFooLogDto {
    fn from(value: &moofoolog::Model) -> Self {
        MooFooLogDto {
            ..Default::default()
        }
    }
}
