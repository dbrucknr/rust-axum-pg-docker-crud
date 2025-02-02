// Native Rust Dependencies
use std::error::Error;
// Third Party Dependencies
use diesel_async::{
    pooled_connection::bb8::Pool, pooled_connection::AsyncDieselConnectionManager,
    AsyncPgConnection,
};

/// Create a new connection pool to a PostgreSQL database
pub async fn build_postgres_pool() -> Result<Pool<AsyncPgConnection>, Box<dyn Error>> {
    let connection_url = std::env::var("DATABASE_URL")?;
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
    let pool = Pool::builder().build(manager).await?;
    Ok(pool)
}
