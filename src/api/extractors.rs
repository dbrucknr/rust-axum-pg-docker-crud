use axum::extract::{FromRef, FromRequestParts};
use axum::http::{request::Parts, StatusCode};
use diesel_async::{pooled_connection::bb8::PooledConnection, AsyncPgConnection};

use super::errors::internal_error;
use super::state::AppState;

pub struct AsyncPostgres(pub PooledConnection<'static, AsyncPgConnection>);

impl FromRequestParts<AppState> for AsyncPostgres {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let conn = app_state.pg.get_owned().await.map_err(internal_error)?;

        Ok(AsyncPostgres(conn))
    }
}
