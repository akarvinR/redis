

use crate::command::{Set, Command};
use crate::redis::server;
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

impl Command for Set {

    fn make(&self) -> String{
        let mut commandString = "set ".to_string();
        let args = &self.args;
        commandString.push_str(&*args.join(" "));
        commandString
    }
    fn execute(&self, server: &mut RedisServer) -> RespData {

        let key = self.args[0].clone();
        let value = self.args[1].clone();
        let mut ttl = u128::MAX;
        let kv_store = server.get_store();
        if self.args.len() > 2{
            ttl = self.args[3].parse::<u128>().unwrap();

        }
        println!("TTL: {}", ttl);
        let mut reply = "OK".to_string();
        kv_store.set(key, value, ttl);

        // kv_store.set(self.args[0].clone(), self.args[1].clone(), ttl);



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