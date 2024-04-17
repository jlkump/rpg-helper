use std::{collections::HashMap, io};

use super::DataView;



pub struct CharacterDataFile {
    character_name: String,
    file: DataFile
}

pub struct GameMasterDataFile {
    game_setting_name: String,
    file: DataFile
}

pub struct GameDataFile {
    game_setting_name: String,
    file: DataFile
}

pub enum DataFile {
    JSON(String) // Path to json file
}

impl DataFile {
    pub fn get_data<'a>(self) -> Result<DataView<'a>, io::Error> {
        todo!()
    }
}

pub struct StoredData {
    game_id_to_data: HashMap<u32, GameDataFile>,
}