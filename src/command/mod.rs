// pub mod command;

pub enum CommandType{
    Set,
    Echo,
    Ping
}


pub trait Command{
    fn execute(&self) -> String;
    fn set_args(&mut self, args: Vec<String>){
        self.args = args;
    }


}

struct Set {
    args: Vec<String>;
}

struct Ping {
    args: Vec<String>;
}

struct Echo {
    args: Vec<String>;
}

pub mod command_factory;
pub mod ping;
pub mod echo;
pub mod set;
