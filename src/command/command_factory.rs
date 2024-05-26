use crate::command::{CommandType, Command, Set, Echo, Ping, Get, Info};

pub fn command_factory(command_type: &String) -> Result<Box<dyn Command>, String>{
    match command_type.to_lowercase().as_str() {
        "set" => {
            return Ok(Box::new(Set{args: Vec::new()}));
        },
        "echo" => {
            return  Ok(Box::new(Echo{args: Vec::new()}));
        },
        "ping" => {
            return Ok(Box::new(Ping{args: Vec::new()}));
        },
        "get" => {
            return Ok(Box::new(Get{args: Vec::new()}));
        },
        "info" => {
            return Ok(Box::new(Info{args: Vec::new()}));
        },
        _ => {
            return Err("Invalid Command in command factory".to_string());
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_command_factory() {
        let set_command = command_factory(CommandType::Set);
        let echo_command = command_factory(CommandType::Echo);
        let ping_command = command_factory(CommandType::Ping);
        assert_eq!(set_command.execute(vec!["key".to_string(), "value".to_string()]), "OK".to_string());
        assert_eq!(echo_command.execute(vec!["Hello".to_string(), "World".to_string()]), "Hello World".to_string());
        assert_eq!(ping_command.execute(vec![]), "PONG".to_string());
    }
}