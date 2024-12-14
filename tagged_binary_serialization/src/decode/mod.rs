use crate::errors::SerializationError;

mod unsigned;
mod signed;
mod structs;
mod string;
mod floats;

pub trait TagDecode {
    const DECODE_TAG: u16;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
    where
        Self: Sized;
}
