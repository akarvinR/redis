use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;
use std::net::{TcpListener, TcpStream};
use crate::command::Command;
use crate::command::command::command_parser;
use crate::redis::kvstore::KvStore;
use crate::redis::server::RedisServer;
use crate::resp::resp_parser::resp_parser;

struct Master{
    server: RedisServer,
    commands: Vec<dyn Command>
}

impl Master{

    fn listen(&self, mut stream: TcpStream){
        loop {
            let mut buffer = [0; 1024];

            stream.read(&mut buffer).unwrap();
            if buffer[0] == 0 {
                break;
            }

            let (data, i) = resp_parser(&buffer, 0);
            let command = command_parser(data).unwrap();
            //wait for reply
            stream.write(reply.encode().as_bytes());
        }

    }
    fn listener(&self) {
        let ipv4 = self.server.config.get("ipv4").unwrap();
        let port = self.server.config.get("port").unwrap();

        let listener = std::net::TcpListener::bind(format!("{}:{}", , port)).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream_) => {
                    thread::spawn(move || {
                        self.listen(stream_);
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }

    }
    fn run(){

    }
}