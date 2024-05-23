use crate::command::{CommandType, Command, Set, Echo, Ping, Get};

pub fn command_factory(command_type: &String) -> Box<dyn Command>{
    match command_type.to_lowercase().as_str() {
        "set" => {
            return Box::new(Set{args: Vec::new()});
        },
        "echo" => {
            return  Box::new(Echo{args: Vec::new()});
        },
        "ping" => {
            return Box::new(Ping{args: Vec::new()});
        },
        "get" => {
            return Box::new(Get{args: Vec::new()});
        }
        _ => {
            println!("Invalid Command in command factory {}", command_type.to_lowercase().as_str());
            panic!("Invalid Command in command factory ")
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