use crate::{errors::SerializationError, type_specification::TypeTag};

use super::TagDecode;

impl TagDecode for u8 {
    const DECODE_TAG: u16 = TypeTag::U8;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
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

        let val_bytes: [u8; 1] = (&bytes[2..3])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let num = Self::from_le_bytes(val_bytes);

        Ok((num, 3))
    }
}

impl TagDecode for u16 {
    const DECODE_TAG: u16 = TypeTag::U16;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
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

        let val_bytes: [u8; 2] = (&bytes[2..4])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

       let num = Self::from_le_bytes(val_bytes);

        Ok((num, 4))
    }
}

impl TagDecode for u32 {
    const DECODE_TAG: u16 = TypeTag::U32;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
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

        let val_bytes: [u8; 4] = (&bytes[2..6])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

       let num = Self::from_le_bytes(val_bytes);

        Ok((num, 6))
    }
}

impl TagDecode for u64 {
    const DECODE_TAG: u16 = TypeTag::U64;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
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

        let val_bytes: [u8; 8] = (&bytes[2..10])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let num = Self::from_le_bytes(val_bytes);

        Ok((num, 10))
    }
}

impl TagDecode for u128 {
    const DECODE_TAG: u16 = TypeTag::U128;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
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

        let val_bytes: [u8; 16] = (&bytes[2..18])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let num = Self::from_le_bytes(val_bytes);

        Ok((num, 18))
    }
}
