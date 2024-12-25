mod routes;
mod model; // for some reason all the pkgs must be declared here in main??

use crate::model::QueryRoot;
use crate::routes::{health, graphql_handler, graphql_playground};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};

#[tokio::main]
async fn main() {
    // i was previously missing dis line oops (the let schema = xx line)
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
       .route("/", get(graphql_playground).post(graphql_handler))
       .route("/health", get(health))
       .layer(Extension(schema));

    Server::bind(&"0.0.0.0:8000".parse().unwrap()) // (3)
        .serve(app.into_make_service())
        .await
        .unwrap();
}