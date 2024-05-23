pub mod bulk_string;
mod array;
pub mod resp;
pub mod resp_parser;
mod infer;
mod parser_factory;
mod null_bulk_string;

pub trait encoder {
    fn encode(&self) -> String  ;
}