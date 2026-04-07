use crate::domain::books::{author_entity::AuthorEntity, create_author_dto::CreateAuthorDto};
use crate::domain::error_handling::books_error::BooksError;
use sqlx::{query, query_as, Error, PgPool};
use tracing::{error, info, warn};

pub async fn fetch_author_by_id(pool: &PgPool, id: i64) -> Result<Option<AuthorEntity>, sqlx::Error> {
    info!("Выполняем запрос автора id={}", id);

    let author = query_as!(AuthorEntity, "SELECT id, name, created_at, updated_at FROM authors WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;

        Ok(author)
}

pub async fn fetch_authors(pool: &PgPool) -> Result<Vec<AuthorEntity>, sqlx::Error> {
  info!("Выполняем запрос авторов");
    let start = std::time::Instant::now();

    let authors = query_as!(AuthorEntity, r#"SELECT id, name, created_at, updated_at FROM authors"#)
        .fetch_all(pool)
        .await?;

    let elapsed = start.elapsed();
    info!(rows = authors.len(), elapsed_ms = elapsed.as_millis(), "Успешно получены авторы");
    Ok(authors)
}

//pub async fn insert_author(author: CreateAuthorDto) -> Result<AuthorEntity, sqlx::Error> {

//}
