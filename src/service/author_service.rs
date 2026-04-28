use crate::domain::books::author_dto::{AuthorDto, CreateAuthorDto};
use crate::domain::error_handling::books_error::BooksError;
use crate::repo::author_repo::{fetch_author_by_id, fetch_authors, save_author};
use crate::state::AppState;

pub async fn get_author_by_id(state: &AppState, id: i64) -> Result<AuthorDto, BooksError> {
    let author_option= fetch_author_by_id(&state.db_pool, id)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))?;

        author_option.map(AuthorDto::from)
    .ok_or(BooksError::NotFound(format!("Автор id = {} не найден", id)))
}

pub async fn get_authors(state: &AppState) -> Result<Vec<AuthorDto>, BooksError> {
    let authors:Result<Vec<AuthorDto>, BooksError> = fetch_authors(&state.db_pool)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
        .map(|authors| {
            authors.into_iter().map(AuthorDto::from).collect()
        });
    authors
}

pub async fn create_author(state: &AppState, author: CreateAuthorDto) -> Result<AuthorDto, BooksError> {
    let author = save_author(&state.db_pool, author)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
        .map(|author_entity| AuthorDto::from(author_entity));

    author
}