use async_graphql::{EmptySubscription, Object, EmptyMutation, Schema};
use lazy_static::lazy_static;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello, world!".to_string()
    }
}

lazy_static! {
  pub(crate) static ref APP_SCHEMA: AppSchema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
}