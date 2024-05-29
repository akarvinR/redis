use std::borrow::Cow;
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::command::Command;
use crate::redis::server::RedisServer;

struct Master{
    tcpStream: TcpStream
}

impl Master{
    fn new(tcpStream: TcpStream) -> Master{
        Master{
            tcpStream: tcpStream
        }
    }

    fn send(&mut self, command: Box<dyn Command>) -> Cow<'_, str> {
        let commandString = command.make();
        self.tcpStream.write(commandString.as_bytes());
        let buffer = &mut [0; 1024];
        self.tcpStream.read(buffer).unwrap();
        let response = String::from_utf8_lossy(buffer);
        println!("Response: {}", response);
        return response;

    }
}

struct  Slave{
    tcpStream: TcpStream,
}
fn handShake() -> bool{

}
fn run(masterIp: String, masterPort: u32){
    let tcpStream = TcpStream::connect(format!("{}:{}", masterIp, masterPort)).unwrap();
    let mut master = Master::new(tcpStream);

    let ok = handShake();
}
