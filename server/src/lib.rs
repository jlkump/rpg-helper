use std::io::Read;

use serde::{Deserialize, Serialize};
use sled::IVec;

mod config;
mod database;
mod api;

pub fn run() {
    println!("Running...");
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TypeData {
    string_data: String,
    vec_data: Vec<String>,
}

impl TypeData {
    fn test_data() -> Self {
        TypeData {
            string_data: "FRUITS".to_owned(),
            vec_data: vec!["APPLE", "BANANA", "CITRUS", "ORANGE"].into_iter().map(|s| s.to_owned()).collect(),
        }
    }

    fn test_data_two() -> Self {
        TypeData {
            string_data: "HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH".to_owned(),
            vec_data: vec!["BIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIGBIG", "Small", "Tiny", "", "Extra"].into_iter().map(|s| s.to_owned()).collect(),
        }
    }
}

pub fn test_many_databases() {
    let tree = sled::open("/tmp/welcome-to-sled").unwrap();
    let id = uuid::uuid!("550e8400-e29b-41d4-a716-446655440000");
    let id_two = uuid::uuid!("550e8400-e28b-41d4-a716-446655440000");
    tree.insert(id, bincode::serialize(&TypeData::test_data()).unwrap());
    tree.insert(id_two, bincode::serialize(&TypeData::test_data_two()).unwrap());

    let data_raw: Vec<u8> = tree.get(&id).unwrap().unwrap().bytes().into_iter().map(|m| m.unwrap()).collect();
    let data_raw_two: Vec<u8> = tree.get(&id_two).unwrap().unwrap().bytes().into_iter().map(|m| m.unwrap()).collect();
    let got: TypeData = bincode::deserialize(data_raw.as_ref()).unwrap();
    let got_two: TypeData = bincode::deserialize(data_raw_two.as_ref()).unwrap();

    println!("Got {:?} from the retrieved data using id: {}", got, id);
    println!("Got {:?} from the retrieved data using id: {}", got_two, id_two);
}