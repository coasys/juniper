use std::{future, pin::Pin};

use coasys_juniper::graphql_subscription;
use futures::{stream, Stream};

type BoxStream<'a, I> = Pin<Box<dyn Stream<Item = I> + Send + 'a>>;

struct Obj;

#[graphql_subscription]
impl Obj {
    async fn id(&self, __num: i32) -> BoxStream<'static, &'static str> {
        Box::pin(stream::once(future::ready("funA")))
    }
}

fn main() {}
