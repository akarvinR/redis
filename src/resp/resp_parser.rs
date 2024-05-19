fn resp_parser(byteArray: &[u8], position: usize) -> (RespData, usize){
    //infer and get from parser factory 
    let mut i = position;
    let rData = RespData::new();   
    
    let (data, new_i) = match Inferer::inferer(byteArray, i) {
        Type::Array => parserFactory(Type::Array)(byteArray, i),
        Type::BulkString => parserFactory(Type::BulkString)(byteArray, i),
    };

    rData.data = data;

    (rData, new_i)


}