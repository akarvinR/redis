pub struct kvstore{
    data: HashMap<String, String>,
    time_to_live: HashMap<String, i64>,

}
impl kvstore {
    pub fn new() -> kvstore {
        kvstore {
            data: HashMap::new(),
            time_to_live: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key.clone(), value.clone());
        self.time_to_live.insert(key.clone(), -1);
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.data.get(&key);
        let time = self.time_to_live.get(&key);
        if time == -1 {
            return self.data.get(&key);
        }
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if current_time < time {
            return self.data.get(&key);
        }
        None
    }

}
