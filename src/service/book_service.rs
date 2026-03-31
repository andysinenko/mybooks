use sqlx::Error as SqlxError;
use crate::domain::books::book::BookEntity;
use crate::domain::books::request_book::BookDto;
use crate::state::AppState;
use crate::repo::book_repo;
use crate::domain::error_handling::books_error::BooksError;

pub async fn get_book(state: &AppState, id: i64) -> Result<BookDto, BooksError> {
    let book  = book_repo::fetch_book(&state.db_pool, id)
        .await
        .map(BookDto::from)
        .ok_or_else(|| (BooksError {message:"Книга не найдена".to_string()}));

    book
}

pub async fn get_books(state: &AppState) -> Vec<BookDto> {
    let books: Result<Vec<BookEntity>, SqlxError> = book_repo::fetch_books(&state.db_pool).await;

    match books {
        Ok(books) => {books.into_iter().map(BookDto::from).collect()},
        Err(error) => {vec![]}
    }

}