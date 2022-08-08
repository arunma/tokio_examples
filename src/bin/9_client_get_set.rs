use mini_redis::client;

// Does not work - Needs message passing
#[tokio::main]
async fn main() {
    let mut client = client::connect("localhost:6379").await.unwrap();

    let t1= tokio::spawn(async {
       let res = client.get("hello").await;
    });

    let t2=tokio::spawn(async {
       client.set("hello", "world".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
