use axum::{extract::State, Json};
use axum::extract::Path;
use tracing::instrument;
use crate::state::AppState;
use crate::service::book_service;
use crate::domain::books::request_book::BookDto;
use crate::domain::error_handling::books_error::BooksError;

#[axum::debug_handler]
#[instrument(skip(state), fields(book_id = %id))]
pub async fn get_book(State(state): State<AppState>, Path(id): Path<i64>) -> Json<Result<BookDto, BooksError>> {
    let book = book_service::get_book(&state, id).await;
    match book {
        Ok(book) => Json(Ok(book)),
        Err(error) => Json(Err(error)),
    }
}

#[axum::debug_handler]
pub async fn get_books(State(state): State<AppState>,) -> Json<Vec<BookDto>> {
    let books = book_service::get_books(&state).await;
    Json(books)
}

pub async fn create_book() {

}

pub async fn update_book() {

}

pub async fn delete_book() {

}



