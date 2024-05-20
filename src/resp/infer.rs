use crate::resp::resp::Type;
pub fn inferer(byte_array: &[u8], position: usize) -> Type{
    // println!("{}", byte_array[position]);
    if byte_array[position] == b'*' as u8 {
        return Type::Array;
    } else if byte_array[position] == b'$' as u8 {
        return Type::BulkString;
    } else {
        panic!("Invalid type");
    }
}

