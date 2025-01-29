// Third Party Dependencies
use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;

// Local Dependencies
mod api;
use api::router::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api = Router::new().nest("/api", routes());

    let listener = TcpListener::bind("[::]:3000").await?;
    // Reachable at Base URL: http://localhost:3000/api
    axum::serve(listener, api).await?;

    Ok(())
}
