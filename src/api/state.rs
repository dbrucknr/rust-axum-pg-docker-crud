use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

#[derive(Clone)]
pub struct AppState {
    pub pg: Pool<AsyncPgConnection>,
}
