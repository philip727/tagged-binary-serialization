use crate::{errors::SerializationError, type_specification::TypeTag};

use super::TagDecode;

impl TagDecode for f32 {
    const DECODE_TAG: u16 = TypeTag::F32;
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

        let bytes: [u8; 4] = (&bytes[2..6])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let num = Self::from_le_bytes(bytes);

        Ok((num, 6))
    }
}

impl TagDecode for f64 {
    const DECODE_TAG: u16 = TypeTag::F64;
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

        let bytes: [u8; 8] = (&bytes[2..10])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let num = Self::from_le_bytes(bytes);

        Ok((num, 10))
    }
}
