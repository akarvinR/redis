

use crate::command::{Get, Command};
use crate::redis::server;
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

impl Command for Get {
    fn execute(&self, server: &mut RedisServer) -> RespData {
        let kv_store = server.get_store();
        let mut reply = "".to_string();

        if self.args.len() == 1 {
            let value = kv_store.get(self.args[0].clone());
            if value.is_some() {
                reply = value.unwrap();
            }
        } else {
            reply = "Invalid Arguments".to_string();
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