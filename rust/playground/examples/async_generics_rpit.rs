use std::time::Duration;
use std::future::Future;

pub trait AsyncDo {
    fn do_thing(&self, thing: String) -> impl Future<Output = String>;
}

struct Client {}

impl AsyncDo for Client {
    async fn do_thing(&self, thing: String) -> String {
        tokio::time::sleep(Duration::from_secs(2)).await;
        format!("from real client {thing}")
    }
}

struct MockClient {}

impl AsyncDo for MockClient {
    async fn do_thing(&self, thing: String) -> String {
        format!("from mock client {thing}")
    }
}

async fn generic_function<A: AsyncDo>(client: A, thing: String) -> String {
    client.do_thing(thing).await
}

#[tokio::main]
async fn main() {
    dbg!(generic_function(Client {}, "hey".into()).await);
    dbg!(generic_function(MockClient {}, "hey".into()).await);
}
