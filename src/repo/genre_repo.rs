use crate::domain::books::genre_entity::GenreEntity;
use crate::domain::error_handling::books_error::BooksError;
use sqlx::{query, query_as, PgPool};
use tracing::{error, info, warn};

pub async fn fetch_genre_by_id(pool: &PgPool, id: i64) -> Result<Option<GenreEntity>, sqlx::Error> {
    info!("Выполняем запрос жанра id={}", id);

    let genre = query_as!(GenreEntity, "SELECT id,
            name,
            note,
            created_at,
            updated_at FROM genre WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;

    Ok(genre)
}

pub async fn fetch_genres(pool: &PgPool) -> Result<Vec<GenreEntity>, sqlx::Error> {
    let start = std::time::Instant::now();

    let genres: Result<Vec<GenreEntity>, sqlx::Error> = query_as!(
        GenreEntity,
        r#"
        SELECT
            id,
            name,
            note,
            created_at,
            updated_at
        FROM genre
        "#
    )
        .fetch_all(pool)
        .await;
    genres
}



