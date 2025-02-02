// Native Rust Dependencies
use std::error::Error;
// Third Party Dependencies
use axum::Router;
use tokio::net::TcpListener;
// Local Dependencies
mod api;
use api::router::routes;
use api::state::AppState;

mod database;
use database::pool::build_postgres_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Postgres Database Connection Pool
    let pool = build_postgres_pool().await?;

    // Axum API
    let state = AppState { pg: pool };
    let api = Router::new().nest("/api", routes()).with_state(state);

    // TCP Socket Listener
    let listener = TcpListener::bind("[::]:3000").await?;

    // API Reachable at Base URL: http://localhost:3000/api
    axum::serve(listener, api).await?;

    // Default Ok Result
    Ok(())
}
