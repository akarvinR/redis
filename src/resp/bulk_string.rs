use crate::resp::resp::RespData;
use crate::resp::resp::Type;
use crate::resp::resp::Data;
pub struct BulkString {
    pub len: usize,
    pub string: String,

}

impl BulkString{
    pub fn encode(&self) -> String{
        format!("${}\r\n{}\r\n", self.len, self.string)
    }
}

pub fn bulk_string_parser(byte_array: &[u8], position: usize) -> (RespData, usize) {
    if position >= byte_array.len() {
        panic!("Invalid BulkString (position >= byteArray.len())");
    }
    let mut bulk_string = BulkString {
        len: 0,
        string: String::new(),
    };
    let mut i = position;
    if byte_array[position] != b'$' as u8 {
        panic!("Invalid BulkString (does not start with $)");
    }
    i += 1;
    let mut bulk_string_size = 0;
    while byte_array[i] != b'\r' {
        bulk_string_size = bulk_string_size * 10 + (byte_array[i] - b'0') as usize;
        i += 1;
    }
    i += 2;

    let mut string = String::new();
    for _ in 0..bulk_string_size {
        string.push(byte_array[i] as char);
        i += 1;
    }
    i += 2;
    bulk_string.len = bulk_string_size;
    bulk_string.string = string;
    // println!("BULK STRING {}", bulk_string.string);
    (RespData{resp_type:Type::BulkString, data: Data::BulkString(bulk_string)}, i)

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_bulk_string_parser() {
        let byte_array = b"$5\r\nhello\r\n";
        let (bulk_string, _) = bulk_string_parser(byte_array, 0);
        let _bulk_string = match bulk_string.data {
            Data::BulkString(b) => {
                assert_eq!(b.len, 5);
                assert_eq!(b.string, "hello");
            },
            _ => panic!("Invalid BulkString"),
        };
     
    }
}
