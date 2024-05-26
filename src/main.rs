// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::*;
use std::thread;
use std::str::from_utf8;
use std::env;
mod resp;
mod command;


use resp::resp_parser::resp_parser;
use crate::resp::resp::Data;

mod redis;
use crate::redis::server::RedisServer;


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();

    let mut port = 6379;
    let mut replicaof = "".to_string();


    for mut i in 1..args.len(){
        if args[i-1] == "--port"{
            port = args[i].parse::<i32>().unwrap();
            println!("Port: {}", port);
            i += 1;
        }
        if args[i-1] == "--replicaof"{
            replicaof = args[i].clone();
            i+=1
        }

    }

    // Uncomment this block to pass the first stage
    let mut server = RedisServer::new("127.0.0.1", port as u32);
    let mut parsedReplicaId =  replicaof.split(" ");
    if parsedReplicaId.clone().count() == 2{
        let ip = parsedReplicaId.next().unwrap();
        let port = parsedReplicaId.next().unwrap().parse::<u32>().unwrap();

        server.role = "slave".to_string();
        server.master_host = ip.to_string();
        server.master_port = port.to_string();
        // let replica = RedisServer::new(ip, port);

        // server.push(replica);
    } else {}
    server.run();
    // let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    

}
