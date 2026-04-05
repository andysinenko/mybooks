use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::domain::books::book_entity::BookEntity;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookDto {
    pub publication_year: Option<NaiveDate>,
    pub title: String,
    pub publisher: Option<String>,
    pub volume_number: Option<String>,
    pub genre_id: Option<i64>,
    pub series_id: Option<i64>,
    pub description: Option<String>,
    pub place_id: Option<i64>
}

impl From<BookEntity> for BookDto {
    fn from(entity: BookEntity) -> Self {
        BookDto {
            publication_year: entity.publication_year,
            title: entity.title,
            publisher: entity.publisher,
            volume_number: entity.volume_number,
            genre_id: entity.genre_id,
            series_id: entity.series_id,
            place_id: entity.place_id,
            description: entity.description,
        }
    }
}

