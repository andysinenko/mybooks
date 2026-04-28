use crate::domain::books::genre_dto::{CreateGenreDto, GenreDto};
use crate::domain::error_handling::books_error::BooksError;
use crate::repo::genre_repo::*;
use crate::repo::genre_repo;
use crate::state::AppState;

pub async fn get_genre_by_id(state: &AppState, id: i64) -> Result<GenreDto, BooksError> {
    let genre_option = genre_repo::fetch_genre_by_id(&state.db_pool, id)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))?;

    genre_option.map(GenreDto::from)
        .ok_or_else(|| BooksError::NotFound(format!("Жанр с id {id} не найдена")))
}

pub async fn get_genres(state: &AppState) -> Result<Vec<GenreDto>, BooksError> {
    let genres:Result<Vec<GenreDto>, BooksError> = fetch_genres(&state.db_pool)
            .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
            .map(|genres| {
                genres.into_iter().map(GenreDto::from).collect()
            });
    genres
}

pub async fn create_genre(state: &AppState, genre_dto: CreateGenreDto) -> Result<GenreDto, BooksError> {
    let created_genre = genre_repo::save_genre(&state.db_pool, genre_dto)
        .await
        .map_err(|_| BooksError::DatabaseError("New genre didn't stored".to_string()))
        .map(|genre| GenreDto::from(genre));

    created_genre
}
