use coasys_juniper_rocket::GraphQLResponse;
use rocket::http::Status;

#[test]
fn test_graphql_response_is_public() {
    let _ = GraphQLResponse(Status::Unauthorized, "Unauthorized".into());
}
