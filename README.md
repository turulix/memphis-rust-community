![Build Status](https://img.shields.io/github/actions/workflow/status/turulix/memphis-rust-community/CD.yml)
![docs.rs](https://img.shields.io/docsrs/memphis-rust-community)
![Crates.io](https://img.shields.io/crates/v/memphis-rust-community?label=version)
![Crates.io](https://img.shields.io/crates/l/memphis-rust-community)
![Crates.io](https://img.shields.io/crates/d/memphis-rust-community)

# Memphis Rust Client

This is an unofficial client for Memphis, written in Rust.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
memphis-rust-community = "0.2.1"
```

## Usage

### Consumer

```rust
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::consumer::MemphisConsumerOptions;
use memphis_rust_community::station::MemphisStationsOptions;

#[tokio::main]
async fn main() {
    let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();

    let station_options = MemphisStationsOptions::new("my-station");
    let station = client.create_station(station_options).await.unwrap();

    let consumer_options = MemphisConsumerOptions::new("my-consumer")
        .with_generate_unique_suffix(true);
    let mut consumer = station.create_consumer(consumer_options).await.unwrap();

    let mut message_receiver = consumer.consume().await.unwrap();
    tokio::spawn(async move {
        loop {
            let msg = message_receiver.recv().await;
            // Do something with the message
            break;
        }
    });
}
```

### Producer

```rust
use memphis_rust_community::memphis_client::MemphisClient;
use memphis_rust_community::producer::MemphisProducerOptions;
use memphis_rust_community::station::MemphisStationsOptions;

#[tokio::main]
async fn main() {
    let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();

    let station_options = MemphisStationsOptions::new("my-station");
    let station = client.create_station(station_options).await.unwrap();

    let producer_options = MemphisProducerOptions::new("my-producer")
        .with_generate_unique_suffix(true);

    let mut producer = station.create_producer(producer_options).await.unwrap();

    let msg = ComposableMessage::new()
        .with_payload("Hello World!")
        .with_header("TestHeader", "TestValue");

    producer.produce(msg).await.unwrap();
}
```

## Supported Features

- ✅ Connection
- ✅ Disconnection
- ✅ Create a station
- ✅ Destroy a station
- ✅ Retention
- ✅ Retention values
- ✅ Storage types

---

- ⚠️ Schemaverse (WIP. Disabled by default via feature flag)
- ❌ Create a new schema
- ❌ Enforce a schema Protobuf
- ✅ Enforce a schema Json
- ❌ Enforce a schema GraphQL
- ❌ Detach a schema

---

- ✅ Produce
- ✅ Add headers
- ✅ Async produce
- ✅ Message ID
- ✅ Destroy a producer
- ✅ Consume
- ✅ Ack a message
- ❌ Fetch
- ✅ Message delay
- ✅ Get Headers
- ✅ Get message sequence number
- ✅ Destroying a Consumer
- ✅ Check if broker is connected
- ✅ Consumer prefetch
