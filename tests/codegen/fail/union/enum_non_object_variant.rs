use coasys_juniper::{GraphQLEnum, GraphQLUnion};

#[derive(GraphQLEnum)]
pub enum Test {
    A,
    B,
}

#[derive(GraphQLUnion)]
enum Character {
    Test(Test),
}

fn main() {}
