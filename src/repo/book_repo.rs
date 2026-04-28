use crate::domain::books::book_dto::CreateBookDto;
use crate::domain::books::book_entity::BookEntity;
use sqlx::{query, query_as, Error, PgPool};
use tracing::info;

pub async fn fetch_books(pool: &PgPool) -> Result<Vec<BookEntity>, Error> {
    info!("Выполняем запрос книг");
    let start = std::time::Instant::now();

    let books = query_as!(
        BookEntity,
        r#"
        SELECT
            b.id,
            b.publication_year,
            b.title,
            b.publisher,
            b.volume_number,

            g.id as genre_id,
            g.name as genre_name,
            g.note as genre_note,

            a.id as author_id,
            a.name as author_name,

            s.id as series_id,
            s.name as series_name,

            b.place_id,
            b.description,
            b.created_at,
            b.updated_at
        FROM books b
        INNER JOIN series s ON b.series_id = s.id
        INNER JOIN authors a ON b.author_id = a.id
        INNER JOIN genre g ON b.genre_id = g.id
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

    let book = query_as!(BookEntity, "SELECT
            b.id,
            b.publication_year,
            b.title,
            b.publisher,
            b.volume_number,

            g.id as genre_id,
            g.name as genre_name,
            g.note as genre_note,

            a.id as author_id,
            a.name as author_name,

            s.id as series_id,
            s.name as series_name,

            b.place_id,
            b.description,
            b.created_at,
            b.updated_at
        FROM books b
        INNER JOIN series s ON b.series_id = s.id
        INNER JOIN authors a ON b.author_id = a.id
        INNER JOIN genre g ON b.genre_id = g.id
        WHERE b.id = $1", id)
        .fetch_optional(pool)
        .await?;

    Ok(book)

}

pub async fn save_book(pool: &PgPool, dto: CreateBookDto, ) -> Result<BookEntity, sqlx::Error> {
    info!("Создаём новую книгу: {}", dto.title);

    let book = query_as!(
        BookEntity,
        r#"
        WITH inserted AS (
            INSERT INTO books (
                title, publication_year, publisher,
                volume_number, author_id, genre_id,
                series_id, place_id, description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
        )
        SELECT
            i.id,
            i.publication_year,
            i.title,
            i.publisher,
            i.volume_number,

            g.id as genre_id,
            g.name as genre_name,
            g.note as genre_note,

            a.id as author_id,
            a.name as author_name,

            s.id as series_id,
            s.name as series_name,

            i.place_id,
            i.description,
            i.created_at,
            i.updated_at
        FROM inserted i
        INNER JOIN genre g ON i.genre_id = g.id
        INNER JOIN authors a ON i.author_id = a.id
        INNER JOIN series s ON i.series_id = s.id
        "#,
        dto.title,
        dto.publication_year,
        dto.publisher,
        dto.volume_number,
        dto.author_id,
        dto.genre_id,
        dto.series_id,
        dto.place_id,
        dto.description,
    )
        .fetch_one(pool)
        .await?;

    info!(id = book.id, "Книга успешно создана");
    Ok(book)
}

