use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct BookEntity {
    pub id: i64,
    pub publication_year: Option<NaiveDate>,
    pub title: String,
    pub publisher: Option<String>,
    pub volume_number: Option<String>,
    pub genre_id: Option<i64>,
    pub series_id: Option<i64>,
    pub place_id: Option<i64>,
    pub description: Option<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>
}