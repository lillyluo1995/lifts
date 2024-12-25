use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)] // means the struct Health can be serialized. in this case into JSON
struct Health {
    healthy: bool
}

// IntoResponse is a key type for axum
pub(crate) async fn health() -> impl IntoResponse {
    let health = Health {
        healthy: true
    };

    (StatusCode::OK, Json(health))
}