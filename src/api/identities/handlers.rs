// Third Party Dependencies
use axum::extract::{Json as ExtractJson, Path, State};
use axum::{debug_handler, http::StatusCode, response::Result, Json};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
// Local Dependencies
use crate::api::identities::schemas::{CreateIdentity, UpdateIdentity};
use crate::api::state::AppState;

use crate::database::models::Identity;
use crate::database::schema::identities;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[debug_handler]
pub async fn all(
    State(state): State<AppState>,
) -> Result<Json<Vec<Identity>>, (StatusCode, String)> {
    let conn = &mut state.pg.get().await.map_err(internal_error)?;
    // SELECT * FROM identities;
    let results = identities::table
        .select(Identity::as_select())
        .load::<Identity>(conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(results))
}

pub async fn one(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<&'static str, (StatusCode, String)> {
    let mut conn = state.pg.get().await;
    println!("GET /identities/{id}");
    Ok("GET /identities/{id}")
}

pub async fn create(
    State(state): State<AppState>,
    ExtractJson(payload): ExtractJson<CreateIdentity>,
) -> String {
    let mut conn = state.pg.get().await;
    // INSERT INTO identities (email, password) VALUES ('email', 'password');
    println!("{:#?} = {:#?}", payload.email, payload.password);
    format!("POST /identities {:#?}", payload)
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateIdentity>,
) -> String {
    let mut conn = state.pg.get().await;
    println!("{:#?} = {:#?}", payload.email, payload.password);
    format!("PUT /identities/{id}, {:#?}", payload)
}

pub async fn remove(Path(id): Path<u32>, State(state): State<AppState>) -> String {
    let mut conn = state.pg.get().await;
    format!("DELETE /identities/{id}")
}
