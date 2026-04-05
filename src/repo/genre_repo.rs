use sqlx::{PgPool, query_as, query, Error};
use tracing::{error, info, warn};
use crate::domain::books::genre_entity::GenreEntity;
use crate::domain::error_handling::books_error::BooksError;

pub async fn fetch_genre_by_id(pool: &PgPool, id: i64) -> Option<GenreEntity> {
    info!("Выполняем запрос жанра id={}", id);

    match query_as::<_, GenreEntity>("SELECT * FROM genre WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!(genre_id = %id, error = %e, "Ошибка при запросе жанра из БД");
            e
        }) {
        Ok(Some(genre)) => Some(genre),
        Ok(None) => None,
        Err(_e) => None,
    }
}

pub async fn fetch_genres(pool: &PgPool) -> Result<Vec<GenreEntity>, BooksError> {
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
    match genres {
        Ok(vec_genres) => {
            let elapsed = start.elapsed();
            info!(rows = vec_genres.len(), elapsed_ms = %elapsed.as_millis(), "Успешно получены все жанры");
            Ok(vec_genres)
        },
        Err(e) => {
            error!(error = %e, "Ошибка при получении списка жанров");
            Ok(vec![])
        }
    }
}



