use crate::resp::bulk_string::BulkString;
use crate::resp::array::Array;
pub enum Data{
    BulkString(BulkString), 
    Array(Array),
}

pub enum Type{
    BulkString, 
    Array, 
  
}
pub struct RespData{
    pub resp_type: Type,
    pub data: Data,

}
