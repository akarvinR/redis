use std::collections::HashMap;
use std::hash::Hash;
use std::net::TcpListener;
use crate::command::{Command, ReplConf};
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

fn parse(mut args: Vec<String>) -> HashMap<String, String> {
    let mut config = HashMap::new();
    for i in 1..args.len() {
        let mut key = args[i - 1].clone();
        let mut value = args[i].clone();
        config.insert(key, value);
    }
    config

}
impl Command for ReplConf{


    fn execute(&self, server: &mut RedisServer) -> RespData {
        let reply = "OK".to_string();
        let config = parse(self.args.clone());
        if config.contains_key("listening-port"){
            let port = config.get("listening-port").unwrap();
            let port = port.parse::<u32>().unwrap();
            let ipv4 = "127.0.0.1";
            let tcp = TcpListener::bind(format!("{}:{}", ipv4, port)).unwrap();
            for stream in tcp.incoming(){
                match stream{
                    Ok(stream) => {
                        server.push(stream);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }

        RespData{
            resp_type: Type::BulkString,
            data: Data::BulkString(BulkString{len: reply.len(),
                string: reply, })
        }
    }
    fn set_args(&mut self, args: Vec<String>){
        self.args = args;
    }
}
