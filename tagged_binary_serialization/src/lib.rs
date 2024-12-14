pub mod decode;
pub mod encode;
pub mod errors;
pub mod type_specification;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    pub use crate::decode::TagDecode;
    pub use crate::encode::TagEncode;



    #[test]
    fn test_all() {
        let mut hashmap = HashMap::new();

        hashmap.insert(0i32, vec![Some(50u32), Some(30)]);
        hashmap.insert(1i32, vec![Some(33u32), Some(13), None, Some(1)]);

        let serialized_map = hashmap.encode();
        println!("{:?}", serialized_map);

        let deserialized_map = HashMap::<i32, Vec<Option<u32>>>::decode(&serialized_map);

        println!("{:?}", deserialized_map);

        let vec: Vec<i32> = vec![2, 4, 5, 3];

        let serialized_vec = vec.encode();

        println!("{:?}", serialized_vec);

        let deserialiezd_vec = Vec::<i32>::decode(&serialized_vec);

        println!("{:?}", deserialiezd_vec);

        let string = Some("Hey".to_string());
        let serialized_str = string.encode();

        println!("{:?}", serialized_str);

        let string = Option::<String>::decode(&serialized_str);

        println!("{:?}", string);

        let num: Option<Option<u32>> = Some(Some(2_058_821));
        let serialized = num.encode();

        println!("{:?}", serialized);

        let deserialized = Option::<Option<u32>>::decode(&serialized);

        println!("{:?}", deserialized);
    }
}
