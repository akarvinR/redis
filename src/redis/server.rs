
use crate::redis::kvstore::KvStore;
use std::net::TcpListener;
use std::io::*;
use std::thread;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
// use crossbeam_utils::thread;
use crate::resp::encoder;

use crate::command::Command;
use crate::command::command::command_parser;
use crate::resp::resp_parser::resp_parser;
use crate::resp::resp::Data;
// use crate::resp::encode;
pub enum Role{
    Master,
    Slave,
}
pub struct RedisServer{
    pub port : u32,
    store: KvStore,
    ipv4: String,
    role: Role,
}



impl RedisServer{
    pub fn new(ipv4: &str, port: u32) -> RedisServer{
        RedisServer{
            port: port, 
            ipv4: ipv4.parse().unwrap(),
            store: KvStore::new(),
            role: Role::Master,
        }
    }

    pub fn get_store(&mut self) -> &mut KvStore{
        &mut self.store
    }
    pub fn run(mut self){
        println!("Server running on port {}", self.port);
        let listener = TcpListener::bind(format!("{}:{}", self.ipv4, self.port)).unwrap();
        // let tempServer = RedisServer::new(&self.ipv4, self.port);

        let serverMutex = Arc::new(Mutex::new(self));

        for stream in listener.incoming() {
            let server = Arc::clone(&serverMutex);


            match stream {
                Ok(mut stream) => {

                        thread::spawn(move || {
                            loop {
                                let mut buffer = [0; 1024];
                                let _ = stream.read(&mut buffer);
                                // println!("buffer: {}", buffer[0]);

                                if buffer[0] == 0 {
                                    break;
                                }
                                // let cloned = Arc::clone(&server);
                                let mut server_t = server.lock().unwrap();
                                let (data, i) = resp_parser(&buffer, 0);

                                let command = command_parser(data); // 1 - command, 2:4 - args
                                let reply = command.execute(&mut server_t);
                                // let reply = command.execute(&mut *server_t);
                                // server_t.port = 123;
                                stream.write(reply.encode().as_bytes());
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