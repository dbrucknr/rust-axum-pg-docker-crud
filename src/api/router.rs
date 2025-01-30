// Third Party Dependencies
use axum::{
    http::{StatusCode, Uri},
    routing::post,
    Router,
};
// Local Dependencies
use crate::api::identities::routes::identities;
use crate::api::state::AppState;

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

pub fn routes() -> Router<AppState> {
    let some_router = Router::new().route("/", post(|| async {}));

    return Router::new()
        .nest("/identities", identities())
        .nest("/some-route", some_router)
        .fallback(fallback);
}
