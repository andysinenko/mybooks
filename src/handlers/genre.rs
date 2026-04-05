use axum::{extract::State, Json};
use axum::extract::Path;
use tracing::instrument;
use crate::state::AppState;

use crate::domain::books::genre_dto::GenreDto;
use crate::domain::error_handling::books_error::BooksError;
use crate::service::genre_service;

#[axum::debug_handler]
#[instrument(skip(state), fields(series_id = %id))]
pub async fn get_genre_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<GenreDto>, BooksError> {
    let genre = genre_service::get_genre_by_id(&state, id).await;
    match genre {
        Ok(genre) => Ok(Json(genre)),
        Err(error) => Err(error),
    }
}

#[axum::debug_handler]
pub async fn get_genres(State(state): State<AppState>,) -> Result<Json<Vec<GenreDto>>, BooksError> {
    let genres = genre_service::get_genres(&state).await;
    match genres {
        Ok(genres) => Ok(Json(genres)),
        Err(error) => Err(error),
    }
}

pub async fn create_genre() {

}





