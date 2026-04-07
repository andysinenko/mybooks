use crate::domain::books::author_dto::AuthorDto;
use crate::domain::error_handling::books_error::BooksError;
use crate::service::author_service;
use crate::state::AppState;
use axum::extract::Path;
use axum::{Json, extract::State};
use tracing::instrument;


#[axum::debug_handler]
#[instrument(skip(state), fields(author_id = %id))]
pub async fn get_author_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<AuthorDto>, BooksError> {
    let author = author_service::get_author_by_id(&state, id).await;
    match author {
        Ok(author) => Ok(Json(author)),
        Err(error) => Err(error),
    }
}

#[axum::debug_handler]
pub async fn get_authors(State(state): State<AppState>,) -> Result<Json<Vec<AuthorDto>>, BooksError> {
    let authors = author_service::get_authors(&state).await;
    match authors {
        Ok(authors) => Ok(Json(authors)),
        Err(error) => Err(error),
    }
}

pub async fn create_author(author: AuthorDto) {
  
}
