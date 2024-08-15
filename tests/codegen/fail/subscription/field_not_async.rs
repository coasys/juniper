use std::{future, pin::Pin};

use coasys_juniper::graphql_subscription;
use futures::{stream, Stream};

type BoxStream<'a, I> = Pin<Box<dyn Stream<Item = I> + Send + 'a>>;

struct ObjA;

#[graphql_subscription]
impl ObjA {
    fn id(&self) -> BoxStream<'static, bool> {
        Box::pin(stream::once(future::ready(true)))
    }
}

fn main() {}
