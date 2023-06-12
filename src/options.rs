pub struct ClientOptions {
    pub host: String,
    pub username: String,
    pub password: String,
    pub connection_token: String,
    pub port: i32,
    pub reconnect: bool,
    pub max_reconnect: i32,
    pub max_reconnect_interval_ms: i32,
    pub timeout_ms: i32,
    pub account_id: i32,
}
