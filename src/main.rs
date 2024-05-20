// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::*;
use std::thread;
use std::str::from_utf8;

mod resp;
mod command;

use crate::command::command::CommandParser;
use resp::resp_parser::resp_parser;
use crate::resp::resp::Data;


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
         
                    for _ in 0..10{
                        let mut buffer = [0; 1024];
                        let _ = stream.read(&mut buffer);
                      
                        let (data, i) = resp_parser(&buffer, 0);
                        let command = CommandParser(data);
                        let reply = command.execute();
                        // println!("reply: {}", reply);
                        stream.write(reply.as_bytes());
                        // let _ = match data.data {
                        //     Data::BulkString(b) => {
                        //        println!("BulkString: {}", b.string);
                        //     },
                        //     Data::Array(a) => {
                        //         for d in a.data {
                        //             match d.data {
                        //                 Data::BulkString(b) => {
                        //                     println!("BulkString: {}", b.string);
                        //                 },
                        //                 _ => panic!("Invalid BulkString"),
                        //             };
                        //         }
                        //     },
                        //     _ => panic!("Invalid BulkString"),
                        // };
                        // // println!("result: {}", s);
                        // let _ = stream.write(b"+PONG\r\n");
                    }
                });
                // loop {
                //     let _ =  stream.read(&mut [0; 1024]);
                //     let _ = stream.write(b"+PONG\r\n");
                // }
                // let _ = stream.write(b"+PONG\r\n");
                // stream.write(b"+PONG\r\n");
   
                // println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
