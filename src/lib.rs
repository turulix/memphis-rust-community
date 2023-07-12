//! # Getting started
//! To get started with Memphis, you need to install the Memphis server. You can find the installation instructions [here](https://memphis.dev/)
//!
//! # Connecting to Memphis and Consuming Messages
//! ```rust
//! use memphis_rust_community::memphis_client::MemphisClient;
//! use memphis_rust_community::consumer::MemphisConsumerOptions;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
//!     let consumer_options = MemphisConsumerOptions::new("my-station", "my-consumer")
//!         .with_generate_unique_suffix(true);
//!     let mut consumer = client.create_consumer(consumer_options).await.unwrap();
//!
//!     consumer.consume().await.unwrap();
//!     tokio::spawn(async move {
//!         loop{
//!             let msg = consumer.message_receiver.recv().await;
//!             // Do something with the message
//!             break;
//!         }
//!     });
//! }
//! ```
#![forbid(unsafe_code)]

pub use request_error::RequestError;

pub mod consumer;
pub mod core;
pub mod memphis_client;
pub mod producer;

pub(crate) mod constants;
pub(crate) mod helper;
pub(crate) mod models;

mod request_error;
