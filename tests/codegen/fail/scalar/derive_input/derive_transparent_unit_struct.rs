use coasys_juniper::GraphQLScalar;

#[derive(GraphQLScalar)]
#[graphql(transparent)]
struct ScalarSpecifiedByUrl;

fn main() {}
