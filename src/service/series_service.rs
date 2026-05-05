use crate::domain::books::author_dto::CreateAuthorDto;
use crate::domain::books::series_dto::{CreateSeriesDto, SeriesDto};
use crate::domain::error_handling::books_error::BooksError;
use crate::repo::series_repo::{fetch_series, save_series_repo};
use crate::repo::series_repo;
use crate::state::AppState;

pub async fn get_series_by_id_service(state: &AppState, id: i64) -> Result<SeriesDto, BooksError> {
    let series_option= series_repo::fetch_series_by_id(&state.db_pool, id)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))?;

    series_option
        .map(SeriesDto::from)
        .ok_or(BooksError::NotFound(format!("Серия с id {id} не найдена")))
}

pub async fn get_series_serice(state: &AppState) -> Result<Vec<SeriesDto>, BooksError> {
    fetch_series(&state.db_pool)
            .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
        .map(|series| {
            series.into_iter().map(SeriesDto::from).collect()
        })
}

pub async fn save_series_serice(state: &AppState, create_series_dto: CreateSeriesDto) -> Result<SeriesDto, BooksError> {
    save_series_repo(&state.db_pool, create_series_dto)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
        .map(|s| SeriesDto::from(s))
}
