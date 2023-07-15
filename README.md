![Build Status](https://img.shields.io/github/actions/workflow/status/turulix/memphis-rust-community/CD.yml)
![docs.rs](https://img.shields.io/docsrs/memphis-rust-community)
![Crates.io](https://img.shields.io/crates/v/memphis-rust-community?label=version)
![Crates.io](https://img.shields.io/crates/l/memphis-rust-community)
![Crates.io](https://img.shields.io/crates/d/memphis-rust-community)

# Memphis Rust Client

This is an unofficial client for Memphis, written in Rust.
This is a work in progress and is not yet ready for production use.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
memphis-rust-community = "0.1.3"
```

## Usage

```rust
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::consumer::memphis_consumer_options::MemphisConsumerOptions;

#[tokio::main]
async fn main() {
    let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
    let consumer_options = MemphisConsumerOptions::new("my-station", "my-consumer");
    let mut consumer = client.create_consumer(consumer_options).await.unwrap();
    
    consumer.consume().await.unwrap();
    tokio::spawn(async move {
        loop{
            let msg = consumer.message_receiver.recv().await;
            // Do something with the message
            break;
        }
    });
}
```

## Supported Features
- :white_check_mark: Connection
- :white_check_mark: Disconnection
- :white_check_mark: Create a station
- :white_check_mark: Destroy a station
- :white_check_mark: Retention
- :white_check_mark: Retention values
- :white_check_mark: Storage types
- :x: Create a new schema
- :x: Enforce a schema Protobuf
- :x: Enforce a schema Json
- :x: Enforce a schema GraphQL
- :x: Detach a schema
- :white_check_mark: Produce
- :white_check_mark: Add headers
- :white_check_mark: Async produce
- :white_check_mark: Message ID
- :white_check_mark: Destroy a producer
- :white_check_mark: Consume
- :x: Context to message handler
- :white_check_mark: Ack a message
- :x: Fetch
- :white_check_mark: Message delay
- :white_check_mark: Get Headers
- :white_check_mark: Get message sequence number
- :white_check_mark: Destroying a Consumer
- :white_check_mark: Check if broker is connected
- :x: Consumer prefetch
