pub mod prelude {
    pub use tagged_binary_serialization::decode::TagDecode;
    pub use tagged_binary_serialization::encode::TagEncode;
    pub use tagged_binary_serialization_derive::{TagEncode, TagDecode};
}

mod tests {
    use crate::prelude::{TagDecode, TagEncode};

    #[derive(TagEncode, TagDecode)]
    #[encode(tag(19))]
    struct Item {

    }

    #[derive(TagEncode, TagDecode)]
    #[encode(tag(18))]
    struct Inventory {
        items: Vec<Item>
    }

    #[derive(TagEncode, TagDecode)]
    #[encode(tag(17))]
    struct Player {
        level: u32,
        name: String,
        inventory: Option<Inventory>,
    }

    #[test]
    fn test_use() {

    }
}
