use crate::handlers::author::{get_author_by_id, get_authors};
use crate::handlers::books::{create_book, delete_book, get_book, get_books, update_book};
use crate::handlers::genre::{create_genre, get_genre_by_id, get_genres};
use crate::handlers::series::{create_series, get_series, get_series_by_id};
use crate::state::AppState;
use axum::{routing::get, Router};

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

pub fn author_routes() -> Router<AppState> {
    Router::new()
        .route("/authors", get(get_authors).post(create_genre))
        .route("/authors/{id}", get(get_author_by_id))
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .nest("/api", books_routes())
        .nest("/api", series_routes())
        .nest("/api", genre_routes())
        .nest("/api", author_routes())
        .with_state(state)
}
