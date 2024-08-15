use std::{future, pin::Pin};

use futures::{future, stream};
use coasys_juniper::{graphql_subscription, GraphQLInputObject};
use futures::{stream, Stream};

type BoxStream<'a, I> = Pin<Box<dyn Stream<Item = I> + Send + 'a>>;

#[derive(GraphQLInputObject)]
struct ObjB {
    id: i32,
}

struct ObjA;

#[graphql_subscription]
impl ObjA {
    async fn id(&self) -> BoxStream<'static, ObjB> {
        Box::pin(stream::once(future::ready(ObjB { id: 34 })))
    }
}

fn main() {}
