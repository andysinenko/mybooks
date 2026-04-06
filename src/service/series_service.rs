use crate::domain::books::series_dto::SeriesDto;
use crate::domain::error_handling::books_error::BooksError;
use crate::state::AppState;
use crate::repo::{series_repo};
use crate::repo::series_repo::{fetch_series};

pub async fn get_series_by_id(state: &AppState, id: i64) -> Result<SeriesDto, BooksError> {
    let series= series_repo::fetch_series_by_id(&state.db_pool, id)
        .await
        .map(SeriesDto::from)
        .ok_or_else(|| BooksError::NotFound(format!("Серия с id {id} не найдена")));

    series
}

pub async fn get_series(state: &AppState) -> Result<Vec<SeriesDto>, BooksError> {
    let series:Result<Vec<SeriesDto >, BooksError > = fetch_series(&state.db_pool)
            .await
            .map(|series| {
                series.into_iter().map(SeriesDto::from).collect()
            });
    series
}
