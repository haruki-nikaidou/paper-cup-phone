use std::num::ParseIntError;
use super::message::queue_trait::MessageQueueStore;
use super::parse_config::time_str_to_seconds;
use super::redis_connect::{RedisConfig, RedisConnection};
use super::message::line_manage::LineManager;
use super::message::redis_queue::RedisQueue;
use super::parse_config::{parse_config, Config, Profile};

const CONFIG_NOT_VALID: &str = "Config is not valid.";
const FAILED_TO_LOAD_CONFIG: &str = "Failed to load config.";
const FAILED_TO_CONNECT_TO_DATABASE: &str = "Failed to connect to database.";

enum Queue{
    Redis(RedisQueue)
}

#[derive(PartialEq)]
enum DatabaseType {
    Redis,
    None,
    Unknown
}

pub struct LoadResult {
    queue: Queue,
    line_manager: LineManager,
    profile: Profile,
}

pub fn load_config() -> Result<LoadResult, (String, String)> {
    let config: Config = match parse_config() {
        Ok(config) => config,
        Err(e) => return Err((FAILED_TO_LOAD_CONFIG.to_string(), e.to_string())),
    };
    let Config { profile, database, config } = config;

    // Get database type
    let database_type = if database.type_ == "redis" {
        DatabaseType::Redis
    } else if database.type_ == "" {
        DatabaseType::None
    } else {
        DatabaseType::Unknown
    };

    // # Note: In this version, only Redis is supported.
    // So, when the database type is not Redis, it will return an error.
    if database_type != DatabaseType::Redis {
        return Err((CONFIG_NOT_VALID.to_string(), "Database type is not supported.".to_string()));
    }


    // convert auto_delete_time from string to u64 (as seconds)
    let auto_delete_time: Option<u64> = if config.auto_delete {
        let time: u64 = match time_str_to_seconds(&config.auto_delete_time) {
            Some(time) => time,
            None => return Err((CONFIG_NOT_VALID.to_string(), "Auto delete time is not valid.".to_string())),
        };
        Some(time)
    } else {
        None
    };

    // connect to database
    let redis_connection = match RedisConnection::new(&RedisConfig {
        url: database.url,
        auto_delete_time,
    }) {
        Ok(connection) => connection,
        Err(e) => return Err((FAILED_TO_CONNECT_TO_DATABASE.to_string(), e.to_string())),
    };

    // create queue
    let queue = match RedisQueue::new(&redis_connection) {
        Ok(queue) => Queue::Redis(queue),
        Err(e) => return Err((FAILED_TO_CONNECT_TO_DATABASE.to_string(), e.to_string())),
    };

    // create line manager
    let line_manager = match LineManager::new(redis_connection) {
        Ok(line_manager) => line_manager,
        Err(e) => return Err((FAILED_TO_CONNECT_TO_DATABASE.to_string(), e.to_string())),
    };

    Ok(LoadResult {
        queue,
        line_manager,
        profile,
    })
}