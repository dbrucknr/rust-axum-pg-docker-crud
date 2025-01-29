// Third Party Dependencies
use axum::extract::{Json, Path};
// Local Dependencies
use crate::api::identities::schemas::{CreateIdentity, UpdateIdentity};

pub async fn all() -> String {
    format!("GET /identities")
}

pub async fn one(Path(id): Path<u32>) -> String {
    format!("GET /identities/{id}")
}

pub async fn create(Json(payload): Json<CreateIdentity>) -> String {
    println!("{:#?} = {:#?}", payload.email, payload.password);
    format!("POST /identities {:#?}", payload)
}

pub async fn update(Path(id): Path<u32>, Json(payload): Json<UpdateIdentity>) -> String {
    println!("{:#?} = {:#?}", payload.email, payload.password);
    format!("PUT /identities/{id}, {:#?}", payload)
}

pub async fn remove(Path(id): Path<u32>) -> String {
    format!("DELETE /identities/{id}")
}
