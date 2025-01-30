// Third Party Dependencies
use axum::extract::{Json, Path, State};
// Local Dependencies
use crate::api::identities::schemas::{CreateIdentity, UpdateIdentity};
use crate::api::state::AppState;

pub async fn all(State(state): State<AppState>) -> String {
    format!("GET /identities")
}

pub async fn one(Path(id): Path<u32>, State(state): State<AppState>) -> String {
    format!("GET /identities/{id}")
}

pub async fn create(State(state): State<AppState>, Json(payload): Json<CreateIdentity>) -> String {
    println!("{:#?} = {:#?}", payload.email, payload.password);
    format!("POST /identities {:#?}", payload)
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateIdentity>,
) -> String {
    println!("{:#?} = {:#?}", payload.email, payload.password);
    format!("PUT /identities/{id}, {:#?}", payload)
}

pub async fn remove(Path(id): Path<u32>, State(state): State<AppState>) -> String {
    format!("DELETE /identities/{id}")
}
