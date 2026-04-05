use axum::{Router, routing::get};
use crate::handlers::books::{get_books, get_book, create_book, update_book, delete_book};
use crate::handlers::genre::{create_genre, get_genre_by_id, get_genres};
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

pub fn genre_routes() -> Router<AppState> {
    Router::new()
        .route("/genres", get(get_genres).post(create_genre))
        .route("/genres/{id}", get(get_genre_by_id))
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", books_routes())
        .nest("/api", series_routes())
        .nest("/api", genre_routes())
        .with_state(state)
}
