use coasys_juniper::GraphQLUnion;

#[derive(GraphQLUnion)]
union Character { id: i32 }

fn main() {}
