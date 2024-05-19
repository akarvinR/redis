struct Array{
    data: Vec<RespData>,
    size: u32,
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
    
    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(b'*');
        result.extend(self.size.to_string().as_bytes());
        result.push(b'\r');
        result.push(b'\n');
        for data in &self.data {
            result.extend(data.encode());
        }
        result
    }

   
}

pub fn arrayParser(byteArray: &[u8], position: usize) -> (Array, u32){

    if position >= byteArray.len() {
        panic!("Invalid Array (position >= byteArray.len())");
    }
    let mut array = Array::new();
    let mut i = position;
    if(byteArray[position] != b'*' as u8){
        panic!("Invalid Array (does not start with *)");
    }
    i += 1;
    let mut arraySize = 0;
    while byteArray[i] != b'\r' {
        arraySize = arraySize * 10 + (byteArray[i] - b'0') as u32;
        i += 1;
    }
    i += 2;


    while i < byteArray.len() -1 as u32 {
        if byteArray[i as usize] == b'\r' && byteArray[(i + 1) as usize] == b'\n' {
            i += 2;
            break;
        }

        if byteArray[i as usize] == b'\r' {
            panic!("Invalid Array (missing \\n)");
        }
        let (data, new_i) = respParser(byteArray, i);
        array.push(data);
        i = new_i;
    }

    i += 1;
    (array, i)

}