use std::sync::{Arc, Mutex};
use redis::Client;

pub struct RedisConfig {
    pub(crate) url: String,
    pub(crate) auto_delete_time: Option<u64>
}

pub struct RedisConnection {
    pub client: Arc<Mutex<Client>>,
    pub auto_delete_time: Option<u64>
}

impl RedisConnection {
    pub fn new(config: &RedisConfig) -> Result<Self, String> {
        let client = Client::open(config.url.as_str()).map_err(|e| e.to_string())?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            auto_delete_time: config.auto_delete_time
        })
    }

    pub fn get_client(&self) -> Arc<Mutex<Client>> {
        self.client.clone()
    }
}