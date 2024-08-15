use std::{future, pin::Pin};

use coasys_juniper::graphql_subscription;
use futures::{stream, Stream};

type BoxStream<'a, I> = Pin<Box<dyn Stream<Item = I> + Send + 'a>>;

struct ObjA;

#[graphql_subscription]
impl ObjA {
    async fn id(&self) -> BoxStream<'static, &'static str> {
        Box::pin(stream::once(future::ready("funA")))
    }

    #[graphql(name = "id")]
    async fn id2(&self) -> BoxStream<'static, &'static str> {
        Box::pin(stream::once(future::ready("funB")))
    }
}

fn main() {}
