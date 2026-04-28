use crate::domain::books::author_dto::{AuthorDto, CreateAuthorDto};
use crate::domain::error_handling::books_error::BooksError;
use crate::service::author_service;
use crate::state::AppState;
use axum::extract::Path;
use axum::{extract::State, Json};
use tracing::instrument;

#[axum::debug_handler]
#[instrument(skip(state), fields(author_id = %id))]
pub async fn get_author_by_id_handler(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<AuthorDto>, BooksError> {
    let author = author_service::get_author_by_id(&state, id).await;
    match author {
        Ok(author) => Ok(Json(author)),
        Err(error) => Err(error),
    }
}

#[axum::debug_handler]
pub async fn get_authors_handler(State(state): State<AppState>,) -> Result<Json<Vec<AuthorDto>>, BooksError> {
    let authors = author_service::get_authors(&state).await;
    match authors {
        Ok(authors) => Ok(Json(authors)),
        Err(error) => Err(error),
    }
}

#[axum::debug_handler]
pub async fn create_author_handler(State(state): State<AppState>, Json(author): Json<CreateAuthorDto>) -> Result<Json<AuthorDto>, BooksError>{
    let author = author_service::create_author(&state, author).await;
    match author {
        Ok(author) => Ok(Json(author)),
        Err(err) => Err(err),
    }
}
