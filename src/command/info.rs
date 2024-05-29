

use crate::command::{Get, Command, Info};
use crate::redis::server;
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

impl Command for Info {
    fn make(&self) -> String{
        let mut commandString = "info ".to_string();
        let args = &self.args;
        commandString.push_str(&*args.join(" "));
        commandString
    }
    fn execute(&self, server: &mut RedisServer) -> RespData {
        // let kv_store = server.get_store();
        // let reply   = "Server is running".to_string();
        let mut reply = "".to_string();


        if(self.args.len() == 0 || self.args.len() > 1){
            reply = "Wrong args".to_string();
        }
        else{

            if self.args[0] == "replication" {
                let num_slaves = server.getConfig("num_slaves", "0").parse::<i32>().unwrap();
                let master_replid = server.getConfig("master_replid", "").to_string();
                let master_repl_offset = server.getConfig("master_repl_offset", "0").parse::<i32>().unwrap();
                let role = server.getConfig("role", "").to_string();
                let keys = ["connected_slaves", "master_replid", "master_repl_offset", "role"];

                let values = [num_slaves.to_string(), master_replid.clone(),
                    master_repl_offset.clone().to_string(), role.clone()];
                for i in 0..keys.len() {
                    reply = format!("{}{}:{}\n", reply, keys[i].clone(), values[i].clone());
                }
            }
            else{
                reply = "Wrong args".to_string();
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