use std::io;
use std::sync::{Arc, Mutex};
use nats::{Connection, Options};
use crate::producer::memphis_producer_options::MemphisProducerOptions;

pub struct MemphisClient {
    broker_connection: Connection,
    is_connected: Arc<Mutex<bool>>,
}

impl MemphisClient {
    pub fn new(
        mut broker_conn_options: Options,
        connection_url: String,
    ) -> io::Result<MemphisClient> {
        let is_connected = Arc::new(Mutex::new(false));
        let is_connected_clone = is_connected.clone();
        let is_connected_clone2 = is_connected.clone();
        let is_connected_clone3 = is_connected.clone();

        broker_conn_options = broker_conn_options.disconnect_callback(move || {
            *is_connected_clone.lock().unwrap() = false;
            println!("Disconnected from NATS");
        }).reconnect_callback(move || {
            *is_connected_clone2.lock().unwrap() = true;
            println!("Reconnected to NATS");
        }).close_callback(move || {
            *is_connected_clone3.lock().unwrap() = false;
            println!("Connection to NATS closed");
        });

        let connection = broker_conn_options.connect(connection_url)?;

        Ok(MemphisClient {
            broker_connection: connection,
            is_connected,
        })
    }

    pub fn CreateProducer(&self, producerOptions: MemphisProducerOptions) {
        let station_name = producerOptions.station_name;
        let producer_name = producerOptions.producer_name;
        let generate_unique_suffix = producerOptions.generate_unique_suffix;
    }
}
