use crate::domain::books::book_entity::BookEntity;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::domain::books::author_dto::AuthorDto;
use crate::domain::books::genre_dto::GenreDto;
use crate::domain::books::series_dto::SeriesDto;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookDto {
    pub publication_year: Option<NaiveDate>,
    pub title: String,
    pub publisher: Option<String>,
    pub volume_number: Option<String>,
    pub author: AuthorDto,
    pub genre: GenreDto,
    pub series: SeriesDto,
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
            author: AuthorDto {
                id: entity.author_id,
                name: entity.author_name,
            },
            genre: GenreDto {
                id: entity.genre_id,
                name: entity.genre_name,
                note: entity.genre_note,
            },
            series: SeriesDto {
                id: entity.series_id,
                name: entity.series_name
            },
            place_id: entity.place_id,
            description: entity.description,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateBookDto {
    pub title: String,
    pub publication_year: Option<NaiveDate>,
    pub publisher: Option<String>,
    pub volume_number: Option<String>,
    pub author_id: i64,
    pub genre_id: i64,
    pub series_id: i64,
    pub place_id: Option<i64>,
    pub description: Option<String>,
}

