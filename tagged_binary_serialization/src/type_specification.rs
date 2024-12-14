pub struct TypeTag;

impl TypeTag {
    pub const U8: u16 = 0;
    pub const U16: u16 = 1;
    pub const U32: u16 = 2;
    pub const U64: u16 = 3;
    pub const U128: u16 = 4;

    pub const I8: u16 = 5;
    pub const I16: u16 = 6;
    pub const I32: u16 = 7;
    pub const I64: u16 = 8;
    pub const I128: u16 = 9;

    pub const F32: u16 = 10;
    pub const F64: u16 = 11;

    pub const STRING: u16 = 12;
    pub const OPTION: u16 = 13;
    pub const VEC: u16 = 14;
    pub const HASHMAP: u16 = 15;

    pub const STD_MAX: u16 = Self::HASHMAP;
}
