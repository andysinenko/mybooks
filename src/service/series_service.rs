use sqlx::Error as SqlxError;
use crate::domain::books::book_entity::BookEntity;
use crate::domain::books::book_dto::BookDto;
use crate::domain::error_handling::books_error::BooksError;
use crate::state::AppState;
use crate::repo::book_repo;

pub async fn get_series_by_id(state: &AppState, id: i64) -> Result<BookDto, BooksError> {
    let book  = book_repo::fetch_book(&state.db_pool, id)
        .await
        .map(BookDto::from)
        .ok_or_else(|| BooksError::NotFound(format!("Книга с id {id} не найдена")));

    book
}

pub async fn get_series(state: &AppState) -> Vec<BookDto> {
    let books: Result<Vec<BookEntity>, SqlxError> = book_repo::fetch_books(&state.db_pool).await;

    match books {
        Ok(books) => {books.into_iter().map(BookDto::from).collect()},
        Err(error) => {vec![]}
    }
}