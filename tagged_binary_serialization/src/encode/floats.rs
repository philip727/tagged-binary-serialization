use crate::type_specification::TypeTag;

use super::TagEncode;

impl TagEncode for f32 {
    const ENCODE_TAG: u16 = TypeTag::F32;
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let le_bytes = self.to_le_bytes();
        bytes.extend(le_bytes);

        bytes
    }
}

impl TagEncode for f64 {
    const ENCODE_TAG: u16 = TypeTag::F64;
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let le_bytes = self.to_le_bytes();
        bytes.extend(le_bytes);

        bytes
    }
}
