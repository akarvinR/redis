use crate::resp::encoder;
use crate::resp::resp::{Type, Data};
use crate::resp::resp::RespData;
use crate::resp::resp_parser::{resp_parser};
pub struct Array{
    pub data: Vec<RespData>,
    pub size: u32,
}
impl Array {
    fn new() -> Array {
        Array {
            data: Vec::new(),
            size: 0,
        }
    }
    fn push(&mut self, data: RespData) {
        self.data.push(data);
        self.size += 1;
    }

    fn get(&self, index: u32) -> Option<&RespData> {
        if index >= self.size {
            return None;
        }
        Some(&self.data[index as usize])
    }

    fn len(&self) -> u32 {
        self.size
    }
    


   
}

impl encoder for Array {
    fn encode(&self) -> String {
        let mut encoded = format!("*{}\r\n", self.size);
        for data in &self.data {
            // let encoded_data = data.encode();
            // encoded.push_str(&data.encode());
        }
        encoded
    }
}

pub fn array_parser(byte_array: &[u8], position: usize) -> (RespData, usize){

    if position >= byte_array.len() {
        panic!("Invalid Array (position >= byte_array.len())");
    }
    let mut array = Array::new();
    let mut i = position;
    if byte_array[position] != b'*'  {
        panic!("Invalid Array (does not start with *)");
    }
    i += 1;
    let mut array_size = 0;
    while byte_array[i] != b'\r' {
        array_size = array_size * 10 + (byte_array[i] - b'0') as u32;
        i += 1;
    }
    i += 2;


    while (i as u32) < byte_array.len() as u32 - 1 {
       
        if byte_array[i as usize] == b'\r' && byte_array[(i + 1) as usize] == b'\n' {
            i += 2;
            break;
        }

        if byte_array[i as usize] == b'\r' {
            panic!("Invalid Array (missing \\n)");
        }

        if byte_array[i as usize] == 0 {

            break;
        }
        let (data, new_i) = resp_parser(byte_array, i);
        array.push(data);
        i = new_i;
    }

    i += 1;
    println!("{}", array.len());
  
    (RespData{resp_type: Type::Array, data: Data::Array(array)}, i)
    // (array, i)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_parser() {
        let byte_array = b"*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        let (respdata, _) = array_parser(byte_array, 0);
        let array = match respdata.data {
            Data::Array(a) => a,
            _ => panic!("Invalid Array"),
        };
        assert_eq!(array.len(), 2);

        let _ = match &array.get(0).unwrap().data {
            Data::BulkString(b) => {
                assert_eq!(b.len, 5);
                assert_eq!(b.string, "hello");
            },
            _ => panic!("Invalid BulkString"),
        };

        let _ = match &array.get(1).unwrap().data {
            Data::BulkString(b) => {
                assert_eq!(b.len, 5);
                assert_eq!(b.string, "world");
            },
            _ => panic!("Invalid BulkString"),
        };


    }
}
