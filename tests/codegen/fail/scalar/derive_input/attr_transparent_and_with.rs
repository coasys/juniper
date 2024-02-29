use coasys_juniper::graphql_scalar;

#[graphql_scalar(with = Self, transparent)]
struct Scalar;

fn main() {}
