pub fn inferer(byteArray: &[u8], position: usize) -> (Type){
    if byteArray[position] == b'*' as u8 {
        return Type::Array;
    } else if byteArray[position] == b'$' as u8 {
        return Type::BulkString;
    } else {
        panic!("Invalid Inferer (does not start with * or $)");
    }
}