use crate::type_specification::TypeTag;

use super::TagEncode;

impl TagEncode for u8 {
    const ENCODE_TAG: u16 = TypeTag::U8;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for u16 {
    const ENCODE_TAG: u16 = TypeTag::U8;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for u32 {
    const ENCODE_TAG: u16 = TypeTag::U32;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for u64 {
    const ENCODE_TAG: u16 = TypeTag::U64;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for u128 {
    const ENCODE_TAG: u16 = TypeTag::U128;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}
