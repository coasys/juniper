use std::convert::Infallible;

use coasys_juniper::graphql_object;

struct Query;

#[graphql_object]
impl Query {
    fn ping() -> Result<bool, Infallible> {
        Ok(false)
    }
}
