use crate::handlers::author::{create_author_handler, get_author_by_id_handler, get_authors_handler};
use crate::handlers::books::{create_book_handler, delete_book_handler, get_book_handler, get_books_handler, update_book_handler};
use crate::handlers::genre::{create_genre_handler, get_genre_by_id_handler, get_genres_handler};
use crate::handlers::series::{create_series_handler, get_series_by_id_handler, get_series_handler};
use crate::state::AppState;
use axum::{routing::get, Router};

pub fn books_routes() -> Router<AppState> {
    Router::new()
        .route("/books", get(get_books_handler).post(create_book_handler))
        .route("/books/{id}", get(get_book_handler).put(update_book_handler).delete(delete_book_handler))
}

pub fn series_routes() -> Router<AppState> {
    Router::new()
        .route("/series", get(get_series_handler).post(create_series_handler))
        .route("/series/{id}", get(get_series_by_id_handler))
}

pub fn genre_routes() -> Router<AppState> {
    Router::new()
        .route("/genres", get(get_genres_handler).post(create_genre_handler))
        .route("/genres/{id}", get(get_genre_by_id_handler))
}

pub fn author_routes() -> Router<AppState> {
    Router::new()
        .route("/authors", get(get_authors_handler).post(create_author_handler))
        .route("/authors/{id}", get(get_author_by_id_handler))
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", books_routes())
        .nest("/api", series_routes())
        .nest("/api", genre_routes())
        .nest("/api", author_routes())
        .with_state(state)
}
