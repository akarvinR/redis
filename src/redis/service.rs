use std::collections::HashMap;
use crate::redis::server::RedisServer;

struct Service{
    redis_server: RedisServer,
}


impl Service{

    pub fn new(args: Vec<String> ) -> Service{
        let mut service = Service{
            redis_server: RedisServer::new(args),
        };
        service
    }
    fn run(&self){
        if self.redis_server.
    }
}

