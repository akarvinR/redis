

use crate::resp::resp::RespData;
use crate::resp::resp::Data;
use crate::resp::bulk_string::BulkString;


pub fn CommandParser(resp: RespData) -> Command{
    let data = resp.data;
    if let Data::Array(array) = data {
        let command_enc = &array.data[0].data;
        let mut args: Vec<String> = Vec::new();
        for arg in array.data.iter().skip(1) {
            match &arg.data {
                Data::BulkString(data) => {
                    args.push(data.string.clone());
                },
                _ => panic!("Invalid Command (NOT BULK STRING)"),
            }
        }
        if let Data::BulkString(command) = command_enc {
            let command = command_factory(command.string)
            return command;
        }else{
            panic!("Invalid Command (NOT BULK STRING)");
        }

    }
    command
}