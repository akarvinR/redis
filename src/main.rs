
use std::io::*;

use std::env;
use std::hash::Hash;

mod resp;
mod command;


mod redis;
use crate::redis::server::RedisServer;




fn main() {

    let args: Vec<String> = env::args().collect();
    let mut server = RedisServer::new(args);
    server.run();

    

}
