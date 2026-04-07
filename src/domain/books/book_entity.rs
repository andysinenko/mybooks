use chrono::{NaiveDate, Utc};
use serde::Serialize;
use sqlx::FromRow;
use crate::domain::books::series_entity::SeriesEntity;

#[derive(Debug, Serialize, FromRow)]
pub struct BookEntity {
    pub id: i64,
    pub publication_year: Option<NaiveDate>,
    pub title: String,
    pub publisher: Option<String>,
    pub volume_number: Option<String>,

    pub author_id: i64,
    pub author_name: String,

    pub genre_id: i64,
    pub genre_name: String,
    pub genre_note: Option<String>,

    pub series_id: i64,
    pub series_name: String,

    pub place_id: Option<i64>,
    pub description: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}