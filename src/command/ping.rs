

use crate::command::{Ping, Command};
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

impl Command for Ping {
    fn execute(&self, server: &mut RedisServer) -> RespData {
        let reply = "PONG".to_string();
        RespData{
            resp_type: Type::BulkString,
            data: Data::BulkString(BulkString{len: reply.len(),
                string: reply, })
        }
        // return "PONG".to_string();
    }


    fn set_args(&mut self, args: Vec<String>){
        self.args = args;
    }
}

