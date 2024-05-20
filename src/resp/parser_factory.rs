use crate::resp::resp::RespData;
use crate::resp::resp::Type;
use crate::resp::array::array_parser;
use crate::resp::bulk_string::bulk_string_parser;

pub fn parser_factory(data_type: Type) -> impl Fn(&[u8], usize) -> (RespData, usize) {

    match data_type {
        Type::Array => array_parser,
        Type::BulkString => bulk_string_parser,
        
    }
}