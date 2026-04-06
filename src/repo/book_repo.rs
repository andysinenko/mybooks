use crate::domain::books::book_entity::BookEntity;
use sqlx::{query, query_as, Error, PgPool};
use tracing::{error, info, warn};

pub async fn fetch_books(pool: &PgPool) -> Result<Vec<BookEntity>, Error> {
    let start = std::time::Instant::now();

    let books = query_as!(
        BookEntity,
        r#"
        SELECT
            id,
            publication_year,
            title,
            publisher,
            volume_number,
            genre_id,
            series_id,
            place_id,
            description,
            created_at,
            updated_at
        FROM books
        "#
    )
    .fetch_all(pool)
    .await?;
    let elapsed = start.elapsed();
    info!(rows = books.len(), elapsed_ms = elapsed.as_millis(), "Успешно получены книги");

    Ok(books)
}

pub async fn fetch_book(pool: &PgPool, id: i64) -> Result<Option<BookEntity>, sqlx::Error> {
    info!("Выполняем запрос книги id={}", id);

    let book = query_as!(BookEntity, "SELECT * FROM books WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;

    Ok(book)

}

