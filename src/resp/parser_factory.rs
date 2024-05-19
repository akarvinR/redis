pub fn parserFactory(type: Type){

    match type {
        Type::Array => arrayParser,
        Type::BulkString => bulkStringParser,
    }
}