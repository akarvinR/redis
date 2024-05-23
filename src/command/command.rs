use crate::command::Command;
use crate::command::command_factory::command_factory;
use crate::resp::resp::RespData;
use crate::resp::resp::Data;
use crate::resp::bulk_string::BulkString;

pub fn command_parser(resp: RespData) -> Box<dyn Command>{
    let data = resp.data;
    if let Data::Array(array) = data {
        //For now, first element is the command
        let command_enc = &array.data[0].data;
        //Rest of the elements are arguments
        // println!("Command Len: {}", array.data.len());
        let mut args: Vec<String> = Vec::new();
        for arg in array.data.iter().skip(1) {
            match &arg.data {
                Data::BulkString(data) => {
                    args.push(data.string.clone());
                },
                _ => panic!("Invalid Args in Command (NOT BULK STRING)"),
            }
        }
        if let Data::BulkString(command) = command_enc {

            let mut command = command_factory(&command.string);
            command.set_args(args);
            return command;
        }else{
            panic!("Invalid Command (NOT BULK STRING)");
        }

    }else{
        command_factory(&"Ping".to_string())
    }

}