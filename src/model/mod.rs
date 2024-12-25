use async_graphql::{Context, Object, Schema};
use async_graphql::{EmptyMutation, EmptySubscription};

pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub(crate) struct QueryRoot; // root of all queries users can use 

#[Object] // hooks me up with async_graphql
impl QueryRoot { // dis my queries
    async fn hello(&self, _ctx: &Context<'_>) -> &'static str {
        "Hello world"
    }
}