use crate::{errors::SerializationError, type_specification::TypeTag};

use super::TagDecode;

impl TagDecode for String {
    const DECODE_TAG: u16 = TypeTag::STRING;
    fn decode(bytes: &[u8]) -> Result<(Self, usize), crate::errors::SerializationError>
    where
        Self: Sized,
    {
        let tag_bytes: [u8; 2] = (&bytes[0..2])
            .try_into()
            .map_err(|_| SerializationError::NoTag)?;

        let tag = u16::from_le_bytes(tag_bytes);

        if tag != Self::DECODE_TAG {
            return Err(SerializationError::MismatchedTag(Self::DECODE_TAG, tag));
        }

        let len_bytes: [u8; 4] = (&bytes[2..6])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let len = u32::from_le_bytes(len_bytes) as usize;

        let str = String::from_utf8(bytes[6..6 + len].to_vec())
            .map_err(|_| SerializationError::NoValue)?;

        Ok((str, 6 + len))
    }
}
