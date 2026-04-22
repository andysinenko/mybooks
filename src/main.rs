mod domain;
mod app;
mod state;
mod handlers;
mod service;
mod repo;

use std::env;
use state::AppState;
use sqlx::PgPool;
use crate::app::create_app;

#[tokio::main]
async fn main() {
    tracer_subscr();
    dotenv::dotenv().ok();

    let database_url = build_db_url();

    tracing::info!("DATABASE_URL {}", database_url);

    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let state = AppState { db_pool };

    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn build_db_url() -> String {
    let user = env::var("POSTGRES_USERNAME_RST")
        .expect("POSTGRES_USERNAME_RST not set");

    let password = env::var("POSTGRES_PASSWORD_RST")
        .expect("POSTGRES_PASSWORD_RST not set");

    let host = env::var("POSTGRES_HOST_RST")
        .unwrap_or_else(|_| "localhost".to_string());

    let port = env::var("POSTGRES_PORT_RST")
        .unwrap_or_else(|_| "5432".to_string());

    let db = env::var("POSTGRES_DB_RST")
        .expect("POSTGRES_DB not set");

    let schema = env::var("POSTGRES_SCHEMA_RST")
        .unwrap_or_else(|_| "public".to_string());

    format!(
        "postgresql://{}:{}@{}:{}/{}?options=-c%20search_path%3D{}",
        user, password, host, port, db, schema
    )

}

fn tracer_subscr() {
    tracing_subscriber::fmt()
        .with_env_filter(
            "info,sqlx::query=debug"
        )
        .with_ansi(true)           
        .init();
}
