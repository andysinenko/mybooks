use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::domain::books::genre_entity::GenreEntity;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct GenreDto {
    pub id: i64,
    pub name: String,
    pub note: Option<String>,
}

impl From<GenreEntity> for GenreDto {
    fn from(entity: GenreEntity) -> Self {
        GenreDto {
            id: entity.id,
            name: entity.name,
            note: entity.note
        }
    }
}
