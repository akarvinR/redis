
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
pub struct KvStore{
    data: HashMap<String, String>,
    time_to_live: HashMap<String, u64>,
}
impl KvStore {
    pub fn new() -> KvStore {
        KvStore{
            data: HashMap::new(),
            time_to_live: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String, time: u64) {
        self.data.insert(key.clone(), value.clone());
        let ttl = if time == u64::MAX {
            u64::MAX
        } else {
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + time
        };
        self.time_to_live.insert(key.clone(),  ttl);
    }

    pub fn get(&mut self, key: String) -> Option<String>{
        println!("Getting key: {}", key);
        let value= self.data.get(&key).unwrap().clone();
        let time = self.time_to_live.get(&key).unwrap().clone();

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if time == u64::MAX {
            println!("Value NONE: {}", value);
            self.time_to_live.remove(&key);
            self.data.remove(&key);
            return None
        }
        if current_time < time {
            println!("Value: {}", value);
            return Some(value);
        }

        println!("current Time {}", current_time);
        println!("current Time {}", time);
        None

    }

}

