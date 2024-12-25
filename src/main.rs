mod routes;
mod model; // for some reason all the pkgs must be declared here in main??

use routes::{health, graphql_handler, graphql_playground};
use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new()
       .route("/", get(graphql_playground).post(graphql_handler))
       .route("/health", get(health));

    Server::bind(&"0.0.0.0:8000".parse().unwrap()) // (3)
        .serve(app.into_make_service())
        .await
        .unwrap();
}