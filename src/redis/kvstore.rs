
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::u128;

pub struct KvStore{
    data: HashMap<String, String>,
    time_to_live: HashMap<String, u128>,
}
impl KvStore {
    pub fn new() -> KvStore {
        KvStore{
            data: HashMap::new(),
            time_to_live: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String, time: u128) {
        self.data.insert(key.clone(), value.clone());
        let ttl = if time == u128::MAX {
            u128::MAX

        } else {
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u128 + time
        };
        self.time_to_live.insert(key.clone(),  ttl);
    }

    pub fn get(&mut self, key: String) -> Option<String>{
        println!("Getting key: {}", key);
        let value_wrapped= self.data.get(&key);
        if value_wrapped.is_none() {
            return None;
        }
        let value = value_wrapped.unwrap().clone();
        let time = self.time_to_live.get(&key).unwrap().clone();

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        if current_time < time {
            println!("Value: {}", value);
            return Some(value);
        }

        println!("current Time {}", current_time);
        println!("current Time {}", time);
        None

    }

}

