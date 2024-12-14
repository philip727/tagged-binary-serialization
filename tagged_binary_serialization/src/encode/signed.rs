use crate::type_specification::TypeTag;

use super::TagEncode;

impl TagEncode for i8 {
    const ENCODE_TAG: u16 = TypeTag::I8;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for i16 {
    const ENCODE_TAG: u16 = TypeTag::I16;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for i32 {
    const ENCODE_TAG: u16 = TypeTag::I32;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for i64 {
    const ENCODE_TAG: u16 = TypeTag::I64;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}

impl TagEncode for i128 {
    const ENCODE_TAG: u16 = TypeTag::I128;

    fn encode(&self) -> Vec<u8> {
        let mut le_tag = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let bytes = self.to_le_bytes();
        le_tag.extend(bytes);

        le_tag
    }
}
