use std::{collections::HashMap, hash::Hash};

use crate::type_specification::TypeTag;

use super::TagEncode;

impl<T> TagEncode for Option<T>
where
    T: TagEncode,
{
    const ENCODE_TAG: u16 = TypeTag::OPTION;
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();

        let Some(val) = self else {
            bytes.extend([0, 0]);
            return bytes;
        };

        bytes.extend([1, 0]);
        bytes.extend(val.encode());

        bytes
    }
}

impl<T> TagEncode for Vec<T>
where
    T: TagEncode,
{
    const ENCODE_TAG: u16 = TypeTag::VEC;
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let len = self.len() as u32;

        bytes.extend(len.to_le_bytes());

        for val in self.iter() {
            let val_bytes = val.encode();
            bytes.extend(val_bytes);
        }

        bytes
    }
}

impl<K, V> TagEncode for HashMap<K, V>
where
    K: TagEncode + Hash + Eq,
    V: TagEncode,
{
    const ENCODE_TAG: u16 = TypeTag::HASHMAP;
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let len = self.len() as u32;

        bytes.extend(len.to_le_bytes());

        for (key, val) in self.iter() {
            let key_bytes = key.encode();
            bytes.extend(key_bytes);
            let val_bytes = val.encode();
            bytes.extend(val_bytes);
        }

        bytes
    }
}
