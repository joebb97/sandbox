use std::time::Duration;

use async_trait::async_trait;

#[async_trait]
pub trait AsyncDo {
    async fn do_thing(&self, thing: String) -> String;
}

struct Client {}

#[async_trait]
impl AsyncDo for Client {
    async fn do_thing(&self, thing: String) -> String {
        tokio::time::sleep(Duration::from_secs(2)).await;
        format!("from real client {thing}")
    }
}

struct MockClient {}

#[async_trait]
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
