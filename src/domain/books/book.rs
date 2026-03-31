use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct BookEntity {
    pub id: i64,
    pub publication_year: Option<NaiveDate>,
    pub title: Option<String>,
    pub publisher: Option<String>,
    pub volume_number: Option<String>,
    pub genre_id: Option<i64>,
    pub series_id: Option<i64>,
    pub description: Option<String>,
    pub place_id: Option<i64>,
}