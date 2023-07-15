//! # Getting started
//! To get started with Memphis, you need to install the Memphis server. You can find the installation instructions [here](https://memphis.dev/)
//!
//! # Connecting to Memphis and Consuming Messages
//! ```rust
//! use memphis_rust_community::memphis_client::MemphisClient;
//! use memphis_rust_community::consumer::MemphisConsumerOptions;
//! use memphis_rust_community::station::MemphisStationsOptions;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = MemphisClient::new("localhost:6666", "root", "memphis").await.unwrap();
//!
//!     let station_options = MemphisStationsOptions::new("my-station");
//!     let station = client.create_station(station_options).await.unwrap();
//!
//!     let consumer_options = MemphisConsumerOptions::new("my-consumer")
//!         .with_generate_unique_suffix(true);
//!     let mut consumer = station.create_consumer(consumer_options).await.unwrap();
//!
//!     let mut message_receiver = consumer.consume().await.unwrap();
//!     tokio::spawn(async move {
//!         loop{
//!             let msg = message_receiver.recv().await;
//!             // Do something with the message
//!             break;
//!         }
//!     });
//! }
//! ```
#![forbid(unsafe_code)]

pub use request_error::RequestError;

pub mod memphis_client;

#[cfg(feature = "consumers")]
pub mod consumer;
#[cfg(feature = "producers")]
pub mod producer;
#[cfg(feature = "schemaverse")]
pub mod schemaverse;

pub mod station;

pub(crate) mod constants;
pub(crate) mod helper;
pub(crate) mod models;
pub(crate) mod station_settings;

mod request_error;
