use axum::extract::{FromRef, State};
use diesel_async::{pooled_connection::bb8::Pool, AsyncPgConnection};

#[derive(Clone)]
pub struct AppState {
    pub pg: Pool<AsyncPgConnection>,
}

impl FromRef<State<AppState>> for AppState {
    fn from_ref(state: &State<AppState>) -> Self {
        // Assuming AppState is stored directly in Axum's state
        state.0.clone()
    }
}
