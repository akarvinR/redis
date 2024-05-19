pub enum Type{
    SimpleString,
    Error,
    Integer,
    BulkString,
    Array,
}
pub struct RespData{
    type: Type,
    data: Vec<Type>,
}

impl Resp {
    
}