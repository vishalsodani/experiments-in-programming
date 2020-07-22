use mini_redis::{client, Result};
#[tokio::main]
pub async fn main() -> Result<()> {
    //redis runs on 6379 which was running on my computer; had to shutdown
    //to enable mini-redis-server on 6379
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("answer from server {:?}", result);


    Ok(())
}
