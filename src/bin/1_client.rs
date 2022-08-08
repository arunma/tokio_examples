use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()>{

    // /opt/homebrew/opt/redis/bin/redis-server /opt/homebrew/etc/redis.conf
    // /brew services restart redis
    let mut client = client::connect("localhost:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    print!("Received value for key {}  -> {:?}", "hello", result);
    Ok(())
}