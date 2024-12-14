mod unsigned;
mod signed;
mod structs;
mod string;
mod floats;

pub trait TagEncode {
    const ENCODE_TAG: u16;
    fn encode(&self) -> Vec<u8>;
} 
