

use crate::resp::resp::RespData;
use crate::resp::resp::Data;
use crate::resp::bulk_string::BulkString;
pub enum Command{
    Ping,
    Echo(String),
    Set(String, String)
}

impl Command{
    pub fn execute(&self) -> String{
        match self {
            Command::Ping => {
                let reply = BulkString {
                    string: "PONG".to_string(),
                    len: 4,
                };
                return reply.encode();
            },
            Command::Echo(data) => {
                let reply = BulkString {
                    string: data.clone(),
                    len: data.len(),
                };
                return reply.encode();
            },
            Command::Set(key, value) => {
                return "OK".to_string();
            }
        }
    }
    // fn validity(&self) -> bool;
}

pub fn CommandParser(resp: RespData) -> Command{

    let command = match resp.data {
        Data::Array(array) => {
            let command = &array.data[0].data;
            let mut args: Vec<String> = Vec::new();
            for arg in array.data.iter().skip(1) {
                match &arg.data {
                    Data::BulkString(data) => {
                        args.push(data.string.clone());
                    },
                    _ => panic!("Invalid Command"),
                }
            }
            match command {
                Data::BulkString(command) => {
                    match command.string.as_str() {
                        "PING" => {
                            Command::Ping
                        },
                        "ECHO" => {
                            let data = args[0].clone();
                            Command::Echo(data)
                        },
                        "SET" => {
                            let key = args[0].clone();
                            let value = args[1].clone();
                            Command::Set(key, value)
                        },
                        _ => panic!("Invalid Command"),
                    }
                },
                _ => panic!("Invalid Command"),
            }
        },
        _ => panic!("Invalid Command"),
    };
    command
}