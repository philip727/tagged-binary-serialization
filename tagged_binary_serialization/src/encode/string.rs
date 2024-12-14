use crate::type_specification::TypeTag;

use super::TagEncode;

impl TagEncode for String {
    const ENCODE_TAG: u16 = TypeTag::STRING;

    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let len = self.len() as u32;
        bytes.extend(len.to_le_bytes());
        bytes.extend(self.as_bytes());

        bytes
    }
}

impl TagEncode for &str {
    const ENCODE_TAG: u16 = TypeTag::STRING;
    
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Self::ENCODE_TAG.to_le_bytes().to_vec();
        let len = self.len() as u32;
        bytes.extend(len.to_le_bytes());
        bytes.extend(self.as_bytes());

        bytes
    }
}
