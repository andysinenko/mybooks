use crate::domain::books::author_entity::AuthorEntity;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct AuthorDto {
    pub id: i64,
    pub name: String,
}

impl From<AuthorEntity> for AuthorDto {
    fn from(entity: AuthorEntity) -> Self {
        AuthorDto {
            id: entity.id,
            name: entity.name
        }
    }
}
