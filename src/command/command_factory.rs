pub fn command_factory(command_type: CommandType) => impl Command{
    match command_type {
        CommandType::Set => {


            return Set{};
            
        },
        CommandType::Echo => {

            return Echo{};
        },
        CommandType::Ping => {
            println!("Ping command");
            return Ping{};
        }
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