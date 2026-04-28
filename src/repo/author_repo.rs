use crate::domain::books::{author_entity::AuthorEntity};
use sqlx::{query_as, PgPool};
use tracing::{info};
use crate::domain::books::author_dto::CreateAuthorDto;

pub async fn fetch_author_by_id(
    pool: &PgPool,
    id: i64,
) -> Result<Option<AuthorEntity>, sqlx::Error> {
    info!("Выполняем запрос автора id={}", id);

    let author = query_as!(
        AuthorEntity,
        "SELECT id, name, created_at, updated_at
        FROM authors WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(author)
}

pub async fn fetch_authors(pool: &PgPool) -> Result<Vec<AuthorEntity>, sqlx::Error> {
    info!("Выполняем запрос авторов");
    let start = std::time::Instant::now();

    let authors = query_as!(
        AuthorEntity,
        r#"SELECT id, name, created_at, updated_at FROM authors"#
    )
    .fetch_all(pool)
    .await?;

    let elapsed = start.elapsed();
    info!(
        rows = authors.len(),
        elapsed_ms = elapsed.as_millis(),
        "Успешно получены авторы"
    );
    Ok(authors)
}

pub async fn save_author(
    pool: &PgPool,
    author: CreateAuthorDto,
) -> Result<AuthorEntity, sqlx::Error> {
    info!("Storing new author");
    let record: AuthorEntity = query_as!(
        AuthorEntity,
        r#"
        INSERT INTO authors ( name ) VALUES ($1)
        RETURNING id, name, created_at, updated_at
     "#,
        author.name
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}
