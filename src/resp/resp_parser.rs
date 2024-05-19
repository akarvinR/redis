use crate::resp::infer::inferer;
use crate::resp::resp::Type;
use crate::resp::resp::RespData;
use crate::resp::parser_factory::parser_factory;
pub fn resp_parser(byte_array: &[u8], position: usize) -> (RespData, usize){
    //infer and get from parser factory 
    let i = position;
    
    let (data, new_i) = match inferer(byte_array, i) {
        Type::Array=> parser_factory(Type::Array)(byte_array, i),
        Type::BulkString => parser_factory(  Type::BulkString)(byte_array, i),
    };



    (data, new_i)


}