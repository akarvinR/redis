
use crate::redis::kvstore::KvStore;
use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::thread;
use std::str::from_utf8;
use std::sync::{Arc, Condvar, Mutex};
// use crossbeam_utils::thread;
use crate::resp::encoder;

use crate::command::Command;
use crate::command::command::command_parser;
use crate::resp::resp_parser::resp_parser;
use crate::resp::resp::Data;
// use crate::resp::encode;
use std::collections::{HashMap, HashSet};
use crate::redis::argsParser::parseArgs;

pub enum ServerStates{
    Master,
    Slave,
}
pub struct RedisServer{
    // port : u32,
    store: KvStore,
    // ipv4: String,
    // role: String,
    slaves: Vec<TcpStream>,
    pub config: HashMap<String, String>,

    // commands: Vec<Box<dyn Command + Sync>>,
    pub(crate) master: Option<TcpStream>,
    role: ServerStates
}



impl RedisServer{
    fn send_ping(&mut self){
        let mut master = self.master.as_ref().unwrap();
        master.write(b"PING");
        let buffer = &mut [0; 1024];
        master.read(buffer).unwrap();
        let response = String::from_utf8_lossy(buffer);
        println!("Response: {}", response);
    }

    fn sendReplConfCapa(&mut self){
        let mut master = self.master.as_ref().unwrap();
        let config = self.config.clone();
        let mut replconf = "REPLCONF".to_string();
        replconf.push_str(" capa psync2");
        master.write(replconf.as_bytes());
        let buffer = &mut [0; 1024];
        master.read(buffer).unwrap();
        let response = String::from_utf8_lossy(buffer);
        println!("Response: {}", response);
    }
    fn sendReplConfListeningPort(&mut self){
        let mut master = self.master.as_ref().unwrap();
        let config = self.config.clone();
        let mut replconf = "REPLCONF".to_string();
        replconf.push_str(&format!(" listening-port {}", self.getConfig("port", "6379")));
        master.write(replconf.as_bytes());
        let buffer = &mut [0; 1024];
        master.read(buffer).unwrap();
        let response = String::from_utf8_lossy(buffer);
        println!("Response: {}", response);
    }

    fn sendPsync(&mut self){
        let mut master = self.master.as_ref().unwrap();
        let config = self.config.clone();
        let mut psync = "PSYNC".to_string();
        psync.push_str(&format!(" {}", self.getConfig("master_replid", "?")));
        psync.push_str(&format!(" {}", self.getConfig("master_repl_offset", "-1")));
        master.write(psync.as_bytes());
        let buffer = &mut [0; 1024];
        master.read(buffer).unwrap();
        let response = String::from_utf8_lossy(buffer);
        println!("Response: {}", response);
    }

    fn handshake(&mut self){
        self.send_ping();
        self.sendReplConfListeningPort();
        self.sendReplConfCapa();
    }
    pub fn getConfig(&self, key: &str, default: &str) -> String{
        match self.config.get(&key.to_string()){
            Some(value) => value.clone(),
            None => default.parse().unwrap(),
        }
    }

    pub fn setRole(&mut self) {
        match self.config.get("role").unwrap().as_str(){
            "master" => self.makeMaster(),
            "slave" => self.makeSlave(),
            _ => self.makeMaster(),
        }
    }
    pub fn new(args: Vec<String>) -> RedisServer{
       let mut server = RedisServer{
            store: KvStore::new(),
            slaves: Vec::new(),
            config: HashMap::new(),
            role: ServerStates::Master,
            master: None,

       };
        server.config = parseArgs(args);
        server.setRole();
        return server;
    }

    pub fn makeMaster(&mut self){
        self.role = ServerStates::Master;
        self.config.insert("role".to_string(), "master".to_string());
    }

    pub fn makeSlave(&mut self){
        self.role = ServerStates::Slave;
        self.config.insert("role".to_string(), "slave".to_string());
    }
    pub fn push(&mut self, server: TcpStream){
        self.slaves.push(server);
    }

    pub fn get_store(&mut self) -> &mut KvStore{
        &mut self.store
    }

     fn listenToClient(&mut self, mut stream: TcpStream){
         loop {
             let mut buffer = [0; 1024];

             stream.read(&mut buffer).unwrap();
             if buffer[0] == 0 {
                 break;
             }

             let (data, i) = resp_parser(&buffer, 0);
             let command = command_parser(data).unwrap();
             let reply = command.execute(self);
             stream.write(reply.encode().as_bytes());
         }

    }
    pub fn run(mut self){

        let ipv4 = self.config.get("ipv4").unwrap();
        let port = self.config.get("port").unwrap();

        println!("Server running on port {}", self.config.get("port").unwrap());

        let listener = TcpListener::bind(format!("{}:{}", ipv4, port)).unwrap();
        let serverMutex = Arc::new(Mutex::new(self));

        for stream in listener.incoming() {
            let server = Arc::clone(&serverMutex);
            match stream {
                Ok(stream_) => {
                        thread::spawn(move || {
                                let mut server = server.lock().unwrap();
                                server.listenToClient(stream_);

                        });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }



    }
}