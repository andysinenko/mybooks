use crate::domain::books::series_entity::SeriesEntity;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct SeriesDto {
    pub id: i64,
    pub name: String,
}

impl From<SeriesEntity> for SeriesDto {
    fn from(entity: SeriesEntity) -> Self {
        SeriesDto {
            id: entity.id,
            name: entity.name
        }
    }
}
