

use crate::command::{Get, Command, Info};
use crate::redis::server;
use crate::redis::server::RedisServer;
use crate::resp::bulk_string::BulkString;
use crate::resp::resp::{Data, RespData, Type};

impl Command for Info {
    fn execute(&self, server: &mut RedisServer) -> RespData {
        // let kv_store = server.get_store();
        // let reply   = "Server is running".to_string();
        let mut reply = "".to_string();


        if(self.args.len() == 0 || self.args.len() > 1){
            reply = "Wrong args".to_string();
        }


        if self.args[0] == "replication"{
            let keys = ["connected_slaves", "master_replid", "master_repl_offset", "role"];
            let values = [server.num_slaves.to_string(), server.master_replid.clone(),
                server.master_repl_offset.clone().to_string(), server.get_role()];
            for i in 0..keys.len(){
                reply = format!("{}{}:{}\n", reply, keys[i].clone(), values[i].clone());
            }
        }
        else{
            reply = "Wrong args".to_string();
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