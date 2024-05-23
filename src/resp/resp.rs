use crate::resp::bulk_string::BulkString;
use crate::resp::array::Array;
use crate::resp::encoder;

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

impl encoder for RespData {
    fn encode(&self) -> String{
        match &self.data {
            Data::BulkString(b) => {
                b.encode()
            },
            Data::Array(a) => {
                a.encode()
            }
        }
    }
}