use actix_web::web::Buf;
use serde::de::DeserializeOwned;
use sled::{Db, Tree};

pub mod user;

pub fn get_data<'a, T, K>(db: &Tree, key: &K) -> Option<T> 
where 
    T: DeserializeOwned, 
    K: std::convert::AsRef<[u8]>
{
    if let Some(data) = db.get(&key).unwrap() {
        Some(bincode::deserialize_from(data.reader()).unwrap())
    } else {
        None
    }
}