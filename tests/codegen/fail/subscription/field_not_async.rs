use std::pin::Pin;

use coasys_juniper::graphql_subscription;

type Stream<'a, I> = Pin<Box<dyn futures::Stream<Item = I> + Send + 'a>>;

struct ObjA;

#[graphql_subscription]
impl ObjA {
    fn id(&self) -> Stream<'static, bool> {
        Box::pin(stream::once(future::ready(true)))
    }
}

fn main() {}
