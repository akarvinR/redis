
use crate::store::kvstore;
use std::net::TcpListener;
use std::io::*;
use std::thread;
use std::str::from_utf8;


use crate::command::command::CommandParser;
use crate::resp::resp_parser::resp_parser;
use crate::resp::resp::Data;
pub enum Role{
    Master,
    Slave,
}
pub struct RedisServer{
    port : u32,
    store: kvstore,
    ipv4: String,
    role: Role,
}



impl RedisServer{
    fn new(ipv4: String, port: u32) -> RedisServer{
        RedisServer{
            port: port, 
            ipv4: ipv4,

            store: kvstore::new(),
            role: Role::Master,
        }
    }
    pub fn run(&mut self){
        println!("Server running on port {}", self.port);
        let listener = TcpListener::bind(format!("{}:{}", self.ipv4, self.port)).unwrap();

        for stream in listener.incoming() {
            
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        loop{
                            let mut buffer = [0; 1024];
                            let _ = stream.read(&mut buffer);
                            println!("buffer: {}", buffer[0]);

                            if buffer[0] == 0 {
                                break;
                            }
                            let (data, i) = resp_parser(&buffer, 0);
                            let command = CommandParser(data);
                            let reply = command.execute();

                            stream.write(reply.as_bytes());
                    
                        }
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}