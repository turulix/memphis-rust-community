use crate::memphis_client::MemphisClient;

mod memphis_client;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = MemphisClient::new(
        "",
        "",
        "",
    ).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    match client {
        Ok(c) => {
            println!("Connected to NATS: {}", c.is_connected().await);
        }
        Err(e) => { println!("Failed to connect to NATS: {}", e) }
    }


}
