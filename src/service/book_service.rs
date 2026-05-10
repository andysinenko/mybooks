use crate::domain::books::book_dto::{BookDto, CreateBookDto};
use crate::domain::error_handling::books_error::BooksError;
use crate::repo::book_repo;
use crate::state::AppState;

pub async fn get_book_by_id(state: &AppState, id: i64) -> Result<BookDto, BooksError> {
    let book_option  = book_repo::fetch_book(&state.db_pool, id)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))?;

    book_option
        .map(BookDto::from)
        .ok_or_else(|| BooksError::NotFound(format!("Книга с id {id} не найдена")))
}

pub async fn get_books(state: &AppState) -> Result<Vec<BookDto>, BooksError> {
    book_repo::fetch_books(&state.db_pool)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
        .map(|books| {
            books.into_iter().map(BookDto::from).collect()
        })
}


pub async fn create_book(state: &AppState, dto: CreateBookDto) -> Result<BookDto, BooksError> {
    book_repo::save_book(&state.db_pool, dto)
        .await
        .map_err(|e| BooksError::DatabaseError(e.to_string()))
        .map(BookDto::from)
}