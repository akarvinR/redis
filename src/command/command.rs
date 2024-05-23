use crate::command::Command;
use crate::command::command_factory::command_factory;
use crate::resp::resp::RespData;
use crate::resp::resp::Data;
use crate::resp::bulk_string::BulkString;

pub fn command_parser(resp: RespData) -> Result<Box<dyn Command>, String>{
    let data = resp.data;
    if let Data::Array(array) = data {

        let command_enc = &array.data[0].data;

        let mut args: Vec<String> = Vec::new();
        for arg in array.data.iter().skip(1) {
            match &arg.data {
                Data::BulkString(data) => {
                    args.push(data.string.clone());
                },
                _ => return Err("Invalid Command (NOT BULK STRING)".to_string()),
            }
        }

        if let Data::BulkString(command) = command_enc {
            let mut command = command_factory(&command.string);
            if let Err(e) = command {
                return Err(e);
            }
            let mut command = command.unwrap();
            command.set_args(args);
            return Ok(command);
        }else{
            return Err("Invalid Command (NOT BULK STRING)".to_string());
        }

    }
    Err("Invalid Command (NOT ARRAY)".to_string())

}