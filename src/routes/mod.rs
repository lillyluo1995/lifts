use crate::model::ServiceSchema; // TODO: why is this being so stupid?? 
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
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

// TODO: idk wtf this is 
pub(crate) async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

pub(crate) async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into() // graphql is an api endpoint w/ a special processor
}