
use std::io::*;

use std::env;
use std::hash::Hash;

mod resp;
mod command;


mod redis;
use crate::redis::server::RedisServer;




fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();
    let mut server = RedisServer::new(args);

    server.run();

    

}
