use std::{collections::HashMap, hash::Hash};

use crate::{errors::SerializationError, type_specification::TypeTag};

use super::TagDecode;

impl<T> TagDecode for Option<T>
where
    T: TagDecode,
{
    const DECODE_TAG: u16 = TypeTag::OPTION;

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

        let some_bytes: [u8; 2] = (&bytes[2..4])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let is_some = some_bytes[0] == 1;
        if !is_some {
            return Ok((None, 4));
        };

        let (value, len) = T::decode(&bytes[4..])?;

        Ok((Some(value), 4 + len))
    }
}

impl<T> TagDecode for Vec<T>
where
    T: TagDecode,
{
    const DECODE_TAG: u16 = TypeTag::VEC;

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

        let len_bytes: [u8; 4] = (&bytes[2..6])
            .try_into()
            .map_err(|_| SerializationError::NoValue)?;

        let len = u32::from_le_bytes(len_bytes) as usize;

        let mut vec = Vec::new();
        let mut starting_byte = 6;
        for _ in 0..len {
            let Ok((val, byte_size)) = T::decode(&bytes[starting_byte..]) else {
                continue;
            };

            starting_byte += byte_size;
            vec.push(val);
        }

        Ok((vec, starting_byte))
    }
}

impl<K, V> TagDecode for HashMap<K, V>
where
    K: TagDecode + Hash + Eq,
    V: TagDecode,
{
    const DECODE_TAG: u16 = TypeTag::HASHMAP;

    fn decode(bytes: &[u8]) -> Result<(Self, usize), SerializationError>
        where
            Self: Sized {

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

        let mut map = HashMap::new();
        let mut starting_byte = 6;
        for _ in 0..len {
            let Ok((key, byte_size)) = K::decode(&bytes[starting_byte..]) else {
                continue;
            };
            starting_byte += byte_size;

            let Ok((val, byte_size)) = V::decode(&bytes[starting_byte..]) else {
                continue;
            };

            starting_byte += byte_size;

            map.insert(key, val);
        }

        
        Ok((map, starting_byte))
    }
}
