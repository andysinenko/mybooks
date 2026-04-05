use axum::{extract::State, Json};
use axum::extract::Path;
use tracing::instrument;
use crate::state::AppState;
use crate::service::series_service;
use crate::domain::books::series_dto::SeriesDto;
use crate::domain::error_handling::books_error::BooksError;

#[axum::debug_handler]
#[instrument(skip(state), fields(series_id = %id))]
pub async fn get_series_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<SeriesDto>, BooksError> {
    let series = series_service::get_series_by_id(&state, id).await;
    match series {
        Ok(series) => Ok(Json(series)),
        Err(error) => Err(error),
    }
}

#[axum::debug_handler]
pub async fn get_series(State(state): State<AppState>,) -> Result<Json<Vec<SeriesDto>>, BooksError> {
    let series = series_service::get_series(&state).await;
    match series {
        Ok(series) => Ok(Json(series)),
        Err(error) => Err(error),
    }
}

pub async fn create_series() {

}





