// Third Party Dependencies
use axum::extract::{Path, State};
use axum::{debug_handler, extract, http::StatusCode, response::Result, Json};
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
// Local Dependencies
use crate::api::errors::internal_error;
use crate::api::state::AppState;

use crate::database::models::{CreateIdentity, Identity, UpdateIdentity};
use crate::database::schema::identities;

#[debug_handler] // Debugging Macro - Remove
pub async fn all(
    State(state): State<AppState>,
) -> Result<Json<Vec<Identity>>, (StatusCode, String)> {
    // Extract a PostgreSQL connection from the application state.
    let conn = &mut state.pg.get().await.map_err(internal_error)?;
    // Generate a SQL Query: SELECT * FROM identities;
    let results = identities::table
        .select(Identity::as_select())
        .load::<Identity>(conn)
        .await
        .map_err(internal_error)?;
    // Default Return: JSON List of Identities | [].
    Ok(Json(results))
}

#[debug_handler] // Debugging Macro - Remove
pub async fn one(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Option<Identity>>, (StatusCode, String)> {
    // Extract a PostgreSQL connection from the application state.
    let conn = &mut state.pg.get().await.map_err(internal_error)?;
    // Generate a SQL Query: SELECT * FROM identities WHERE id = $1;
    let result = identities::table
        .find(id)
        .select(Identity::as_select())
        .first::<Identity>(conn)
        .await
        .optional()
        .map_err(internal_error)?;
    // Default Return: JSON Identity | null.
    Ok(Json(result))
}

pub async fn create(
    State(state): State<AppState>,
    extract::Json(payload): extract::Json<CreateIdentity>,
) -> Result<Json<Identity>, (StatusCode, String)> {
    let conn = &mut state.pg.get().await.map_err(internal_error)?;
    // INSERT INTO identities (email, password) VALUES ('email', 'password');
    let result = diesel::insert_into(identities::table)
        .values(&payload)
        .returning(Identity::as_returning())
        .get_result(conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(result))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    extract::Json(payload): extract::Json<UpdateIdentity>,
) -> Result<Json<Identity>, (StatusCode, String)> {
    let conn = &mut state.pg.get().await.map_err(internal_error)?;
    let target = identities::table.filter(identities::id.eq(id));
    let result = diesel::update(target)
        .set(&payload)
        .get_result(conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(result))
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<String>, (StatusCode, String)> {
    let conn = &mut state.pg.get().await.map_err(internal_error)?;
    let target = identities::table.filter(identities::id.eq(id));
    let result = diesel::delete(target)
        .execute(conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(format!("DELETED: {} Identities", result)))
}
