use std::{future, pin::Pin};

use futures::{future, stream};
use coasys_juniper::{graphql_subscription, GraphQLObject};
use futures::{stream, Stream};

type BoxStream<'a, I> = Pin<Box<dyn Stream<Item = I> + Send + 'a>>;

#[derive(GraphQLObject)]
struct ObjA {
    test: String,
}

struct ObjB;

#[graphql_subscription]
impl ObjB {
    async fn id(&self, obj: ObjA) -> BoxStream<'static, &'static str> {
        Box::pin(stream::once(future::ready("funA")))
    }
}

fn main() {}
