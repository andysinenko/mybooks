use sqlx::{PgPool, query_as, query, Error};
use tracing::{error, info, warn};
use crate::domain::books::series_entity::SeriesEntity;
use crate::domain::error_handling::books_error::BooksError;

pub async fn fetch_series_by_id(pool: &PgPool, id: i64) -> Option<SeriesEntity> {
    info!("Выполняем запрос серии id={}", id);

    match query_as::<_, SeriesEntity>("SELECT * FROM series WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            error!(series_id = %id, error = %e, "Ошибка при запросе серии из БД");
            e
        }) {
        Ok(Some(series)) => Some(series),
        Ok(None) => None,
        Err(_e) => None,
    }
}

pub async fn fetch_series(pool: &PgPool) -> Result<Vec<SeriesEntity>, BooksError> {
  info!("Выполняем запрос серий");
    let start = std::time::Instant::now();

    let series = query_as!(
        SeriesEntity,
        r#"
        SELECT
            id,
            name,
            created_at,
            updated_at
        FROM series
        "#
    )
        .fetch_all(pool)
        .await;
    match series {
        Ok(series) => {
            let elapsed = start.elapsed();
            info!(rows = series.len(), elapsed_ms = %elapsed.as_millis(), "Успешно получены все серии");
            Ok(series)
        },
        Err(e) => {
            error!(error = %e, "Ошибка при получении списка серий");
            Ok(vec![])
        }
    }
}
