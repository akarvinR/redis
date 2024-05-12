pub trait ClientDecoder {
    fn decode(&self, data: &[u8]) -> Result<ClientMessage, ClientError>;
}

pub trait ClientEncoder {
    fn encode(&self, message: &str) -> Vec<u8>;
}


