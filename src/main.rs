mod domain;
mod app;
mod state;
mod handlers;
mod service;
mod repo;

use state::AppState;
use sqlx::PgPool;
use crate::app::create_app;

#[tokio::main]
async fn main() {
    tracer_subscr();

    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let state = AppState { db_pool };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn tracer_subscr() {
    tracing_subscriber::fmt()
        .with_env_filter(
            "info,sqlx::query=debug"
        )
        .with_ansi(true)           
        .init();
}
