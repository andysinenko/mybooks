use axum::{Router, routing::get};
use crate::handlers::books::{get_books, get_book, create_book, update_book, delete_book};
use crate::handlers::series::{get_series, get_series_by_id, create_series};
use crate::state::AppState;

pub fn books_routes() -> Router<AppState> {
    Router::new()
        .route("/books", get(get_books).post(create_book))
        .route("/books/{id}", get(get_book).put(update_book).delete(delete_book))
}

pub fn series_routes() -> Router<AppState> {
    Router::new()
        .route("/series", get(get_series).post(create_series))
        .route("/series/{id}", get(get_series_by_id))
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", books_routes())
        .nest("/api", series_routes())
        .with_state(state)
}
