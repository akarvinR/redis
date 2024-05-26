// pub mod command;

use crate::redis::server::RedisServer;
use crate::resp::resp::RespData;

pub enum CommandType{
    Set,
    Echo,
    Ping,
    Get
}
pub trait Command{
    //Execute on Server

    fn execute(&self, redis_server: &mut RedisServer) -> RespData;
    fn set_args(&mut self, args: Vec<String>);

}

struct Set {
    args: Vec<String>
}

struct Ping {
    args: Vec<String>
}

struct Get {
    args: Vec<String>
}

struct Echo {
    args: Vec<String>
}
struct Info {
    args: Vec<String>
}

pub mod command_factory;
pub mod ping;
pub mod echo;
pub mod set;
pub mod command;
mod get;
mod info;
