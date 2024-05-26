
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

pub struct RedisServer{
    port : u32,
    store: KvStore,
    ipv4: String,
    pub role: String,
    servers: Vec<RedisServer>,

    pub master_host: String,
    pub master_port: String,
    pub master_replid: String,
    pub master_repl_offset: u32,
    pub num_slaves: u32,
    // commands: Vec<Box<dyn Command + Sync>>,
}



impl RedisServer{

    pub fn get_role(&self) -> String{
        self.role.clone()
    }


    pub fn new(ipv4: &str, port: u32) -> RedisServer{
        RedisServer{
            port: port, 
            ipv4: ipv4.parse().unwrap(),
            store: KvStore::new(),
            role: "master".to_string(),
            servers: Vec::new(),
            master_replid: "".to_string(),
            master_repl_offset: 0,
            num_slaves: 0,
            master_host: "".to_string(),
            master_port: "".to_string(),

            // commands: Vec::new(),
        }
    }

    pub fn get_store(&mut self) -> &mut KvStore{
        &mut self.store
    }
    pub fn run(mut self){
        println!("Server running on port {}", self.port);
        let listener = TcpListener::bind(format!("{}:{}", self.ipv4, self.port)).unwrap();
        let serverMutex = Arc::new(Mutex::new(self));
        for stream in listener.incoming() {
            let server = Arc::clone(&serverMutex);

            match stream {
                Ok(mut stream) => {

                        thread::spawn(move || {
                            loop {
                                println!("Connection established");
                                let mut buffer = [0; 1024];


                                stream.read(&mut buffer).unwrap();
                                if buffer[0] == 0 {
                                    break;
                                }
                                let mut server_t = server.lock().unwrap();
                                let (data, i) = resp_parser(&buffer, 0);
                                let command = command_parser(data).unwrap();
                                let reply = command.execute(&mut server_t);
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