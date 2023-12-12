use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:26379").await?;

    // Set the key "hello" with value "world"
    // client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    let result = client.get("hello").await?;
    println!("2 - got value from the server; result={:?}", result);

    Ok(())
}
