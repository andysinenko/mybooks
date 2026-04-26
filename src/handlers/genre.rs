use axum::{extract::State, Json};
use axum::extract::Path;
use sqlx::query_as;
use tracing::instrument;
use crate::domain::books::genre_dto;
use crate::state::AppState;

use crate::domain::books::genre_dto::{CreateGenreDto, GenreDto};
use crate::domain::books::genre_entity::GenreEntity;
use crate::domain::error_handling::books_error::BooksError;
use crate::service::genre_service;

#[axum::debug_handler]
#[instrument(skip(state), fields(genre_id = %id))]
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

#[axum::debug_handler]
pub async fn create_genre(State(state): State<AppState>, Json(genre_dto): Json<CreateGenreDto>) -> Result<Json<GenreDto>, BooksError>{
    let genre = genre_service::create_genre(&state, genre_dto).await;
    match genre {
        Ok(genres) => Ok(Json(genres)),
        Err(error) => Err(error),
    }
}
