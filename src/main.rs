// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::*;
use std::thread;
use std::str::from_utf8;

mod resp;
mod command;
mod store;

use crate::command::command::CommandParser;
use resp::resp_parser::resp_parser;
use crate::resp::resp::Data;

mod redis;
use crate::redis::server::RedisServer;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let mut server = RedisServer::new("127.0.0.1", 6379);
    server.run();
    // let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    // for stream in listener.incoming() {
    //     println!("Connection established!");
    //     match stream {
    //         Ok(mut stream) => {
    //             thread::spawn(move || {
         
    //                 loop{
    //                     let mut buffer = [0; 1024];
    //                     let _ = stream.read(&mut buffer);
    //                     println!("buffer: {}", buffer[0]);

    //                     if buffer[0] == 0 {
    //                         break;
    //                     }
    //                     let (data, i) = resp_parser(&buffer, 0);
    //                     let command = CommandParser(data);
    //                     let reply = command.execute();
    //                     // println!("reply: {}", reply);
    //                     stream.write(reply.as_bytes());
                
    //                 }
    //             });
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}
