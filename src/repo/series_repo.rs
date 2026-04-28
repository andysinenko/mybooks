use crate::domain::books::series_entity::SeriesEntity;
use sqlx::{ query_as, PgPool};
use tracing::info;

pub async fn fetch_series_by_id(pool: &PgPool, id: i64) -> Result<Option<SeriesEntity>, sqlx::Error> {
    info!("Выполняем запрос серии id={}", id);

    let series = query_as!(SeriesEntity, "SELECT id, name, created_at, updated_at FROM series WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;

    Ok(series)
}

pub async fn fetch_series(pool: &PgPool) -> Result<Vec<SeriesEntity>, sqlx::Error> {
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
        .await?;
    let elapsed = start.elapsed();
    info!(rows = series.len(), elapsed_ms = elapsed.as_millis(), "Успешно получены серии");

    Ok(series)
}
