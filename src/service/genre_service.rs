use crate::domain::books::genre_dto::GenreDto;
use crate::domain::error_handling::books_error::BooksError;
use crate::state::AppState;
use crate::repo::{genre_repo};
use crate::repo::genre_repo::*;

pub async fn get_genre_by_id(state: &AppState, id: i64) -> Result<GenreDto, BooksError> {
    let genre= genre_repo::fetch_genre_by_id(&state.db_pool, id)
        .await
        .map(GenreDto::from)
        .ok_or_else(|| BooksError::NotFound(format!("Жанр с id {id} не найдена")));

    genre
}

pub async fn get_genres(state: &AppState) -> Result<Vec<GenreDto>, BooksError> {
    let genres:Result<Vec<GenreDto>, BooksError> = fetch_genres(&state.db_pool)
            .await
            .map(|genres| {
                genres.into_iter().map(GenreDto::from).collect()
            });
    genres
}
