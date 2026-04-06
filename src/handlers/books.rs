use axum::{extract::State, Json};
use axum::extract::Path;
use tracing::instrument;
use crate::state::AppState;
use crate::service::{book_service};
use crate::domain::books::book_dto::BookDto;
use crate::domain::error_handling::books_error::BooksError;

#[axum::debug_handler]
#[instrument(skip(state), fields(book_id = %id))]
pub async fn get_book(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<BookDto>, BooksError> {
    let book = book_service::get_book_by_id(&state, id).await;
    match book {
        Ok(book) => Ok(Json(book)),
        Err(error) => Err(error),
    }
}

#[axum::debug_handler]
pub async fn get_books(State(state): State<AppState>,) -> Result<Json<Vec<BookDto>>, BooksError> {
    let books = book_service::get_books(&state).await;
    match books {
        Ok(books) => Ok(Json(books)),
        Err(error) => Err(error),
    }
}

pub async fn create_book() {

}

pub async fn update_book() {

}

pub async fn delete_book() {

}



