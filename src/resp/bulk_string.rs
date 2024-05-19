pub struct BulkString {
    len: usize,
    string: String,
}

pub fn bulk_string_parser(byte_array: &[u8], position: usize) -> (BulkString, usize) {
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
    (bulk_string, i)

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_bulk_string_parser() {
        let byte_array = b"$5\r\nhello\r\n";
        let (bulk_string, _) = bulk_string_parser(byte_array, 0);
        assert_eq!(bulk_string.len, 5);
        assert_eq!(bulk_string.string, "hello");
    }
}
