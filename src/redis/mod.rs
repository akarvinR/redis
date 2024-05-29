use std::collections::HashMap;

pub mod server;
pub mod kvstore;
pub mod argsParser;
mod slaveService;
mod masterService;


pub fn getDefaultConfig () -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("port".to_string(), "6379".to_string());
    config.insert("role".to_string(), "master".to_string());
    config.insert("master_host".to_string(), "".to_string());
    config.insert("master_port".to_string(), "0".to_string());
    config.insert("num_slaves".to_string(), "0".to_string());
    config.insert("master_replid".to_string(), "?".to_string());
    config.insert("master_repl_offset".to_string(), "-1".to_string());
    config.insert("ipv4".to_string(), "127.0.0.1".to_string());
    config.insert("port".to_string(), "6379".to_string());
    config
}