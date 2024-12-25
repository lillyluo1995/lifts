mod routes;

use routes::{health, graphql_handler, graphql_playround};
use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new()
       .route("/", post(graphql_playground).post(graphql_handler))
       .route("/health", get(health));

    Server::bind(&"0.0.0.0:8000".parse().unwrap()) // (3)
        .serve(app.into_make_service())
        .await
        .unwrap();
}