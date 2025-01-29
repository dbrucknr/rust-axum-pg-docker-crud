// Third Party Dependencies
use axum::{routing::delete, routing::get, routing::post, routing::put, Router};
// Local Dependencies
use super::handlers::{all, create, one, remove, update};

pub fn identities() -> Router {
    Router::new()
        // GET http://localhost:3000/api/identities
        .route("/", get(all))
        // GET http://localhost:3000/api/identities/{id}
        .route("/{id}", get(one))
        // POST http://localhost:3000/api/identities
        .route("/", post(create))
        // PUT http://localhost:3000/api/identities/{id}
        .route("/{id}", put(update))
        // DELETE http://localhost:3000/api/identities/{id}
        .route("/{id}", delete(remove))
}
