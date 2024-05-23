

use crate::command::{Set, Command};
use crate::redis::server;
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

impl Command for Set {
    fn execute(&self, server: &mut RedisServer) -> RespData {
        let kv_store = server.get_store();
        let mut reply = "OK".to_string();
        if self.args.len() == 2 {
            kv_store.set(self.args[0].clone(), self.args[1].clone(), u64::MAX);
        } else if self.args.len() == 3 {
            let ttl = self.args[2].parse::<i64>().unwrap();
            kv_store.set(self.args[0].clone(), self.args[1].clone(), ttl as u64);
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