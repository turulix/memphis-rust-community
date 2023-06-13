use std::io;
use std::io::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use nats::asynk::{Connection, Options};
use tokio::runtime::Runtime;
use uuid::Uuid;

pub struct MemphisClient {
    broker_connection: Connection,
    is_connected: Arc<Mutex<bool>>,
}

impl MemphisClient {
    pub async fn new(memphis_host: &str,
                     memphis_username: &str,
                     memphis_password: &str,
    ) -> io::Result<MemphisClient> {
        let is_connected = Arc::new(Mutex::new(false));

        // TODO: Replace 1 with account_id
        let broker_settings = MemphisClient::create_settings(
            is_connected.clone(),
            format!("{}${}", memphis_username, 1).as_str(),
            memphis_password,
        );

        let connection = match broker_settings.connect(memphis_host).await {
            Ok(c) => c,
            Err(e) => {
                if e.to_string().contains("Authorization Violation") {
                    let broker_settings = MemphisClient::create_settings(
                        is_connected.clone(),
                        memphis_username,
                        memphis_password,
                    );
                    let connection = broker_settings.connect(memphis_host).await;
                    match connection {
                        Ok(c) => c,
                        Err(e) => {
                            return Err(e);
                        }
                    }
                } else {
                    return Err(e);
                }
            }
        };

        *is_connected.lock().await = true;

        Ok(MemphisClient {
            broker_connection: connection,
            is_connected,
        })
    }

    pub async fn is_connected(&self) -> bool {
        *self.is_connected.lock().await
    }

    fn create_settings(is_connected: Arc<Mutex<bool>>, memphis_username: &str, memphis_password: &str) -> Options {
        let is_connected2 = is_connected.clone();
        let is_connected3 = is_connected.clone();
        let is_connected4 = is_connected.clone();

        return Options::with_user_pass(memphis_username, memphis_password)
            .disconnect_callback(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(MemphisClient::disconnect_callback(is_connected2.clone()));
            })
            .reconnect_callback(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(MemphisClient::reconnect_callback(is_connected3.clone()));
            })
            .close_callback(move || {
                let rt = Runtime::new().unwrap();
                rt.block_on(MemphisClient::close_callback(is_connected4.clone()));
            })
            .with_name(&*format!("{}::{}", Uuid::new_v4(), memphis_username));
    }

    fn get_connection(&self) -> &Connection {
        &self.broker_connection
    }

    async fn disconnect_callback(is_connected: Arc<Mutex<bool>>) {
        *is_connected.lock().await = false;
    }

    async fn reconnect_callback(is_connected: Arc<Mutex<bool>>) {
        *is_connected.lock().await = true;
    }

    async fn close_callback(is_connected: Arc<Mutex<bool>>) {
        *is_connected.lock().await = false;
    }

// pub fn CreateProducer(&self, producerOptions: MemphisProducerOptions) {
//     let station_name = producerOptions.station_name;
//     let producer_name = producerOptions.producer_name;
//     let generate_unique_suffix = producerOptions.generate_unique_suffix;
// }
}
