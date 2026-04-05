use sqlx::{PgPool, query_as, query, Error};
use tracing::{error, info, warn};
use crate::domain::books::book_entity::BookEntity;

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
        .await;
    match books {
        Ok(books) => {
            let elapsed = start.elapsed();
            info!(rows = books.len(), elapsed_ms = %elapsed.as_millis(), "Успешно получены все книги");
            Ok(books)
        },
        Err(e) => {
            error!(error = %e, "Ошибка при получении списка всех книг");
            Ok(vec![])
        }
    }
}

pub async fn fetch_book(pool: &PgPool, id: i64) -> Option<BookEntity> {
    info!("Выполняем запрос книги id={}", id);

    match query_as::<_, BookEntity>("SELECT * FROM books WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!(book_id = %id, error = %e, "Ошибка при запросе книги из БД");
            e
        }) {
            Ok(Some(book)) => Some(book),
            Ok(None) => None,
            Err(_e) => None,
        }
}

