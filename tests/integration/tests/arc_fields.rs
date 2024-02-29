use std::sync::Arc;

use coasys_juniper::{graphql_object, GraphQLInputObject};

struct Query;

#[graphql_object]
impl Query {
    fn ping() -> Arc<bool> {
        Arc::new(false)
    }
}

#[derive(GraphQLInputObject)]
struct Ping {
    expect_result: Arc<bool>,
}
