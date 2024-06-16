use actix_web::web::Buf;
use serde::de::DeserializeOwned;
use sled::Tree;

use crate::config::Config;

pub mod user;

fn get_data<'a, T, K>(db: &Tree, key: &K) -> Option<T> 
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

pub struct Database {
    pub user_db: user::UserDB,
}

impl Database {
    pub fn open(config: &Config) -> Database {
        Database {
            user_db: user::UserDB::open(&config)
        }
    }
}