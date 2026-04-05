use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow)]
pub struct AuthorDto {
    pub id: i64,
    pub name: String,
}