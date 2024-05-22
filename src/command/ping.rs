

impl Command for Ping {
    fn execute(&self) -> String {
        return "PONG".to_string();
    }
}