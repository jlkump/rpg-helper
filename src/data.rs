use std::{collections::HashMap, io, net::TcpStream};

use crate::network::{AuthorityInstance, ClientInstance};

use self::{file_storage::{CharacterDataFile, GameMasterDataFile}, indexes::Index, meta_type::{MetaType, MetaTypeInstance}};

pub mod meta_type;
pub mod equation;
pub mod timeline;
pub mod indexes;
pub mod dice;
pub mod file_storage;
pub mod modifier;


pub struct AppData<'a> {
    client: Option<ClientInstance<'a>>, // None when not connected to a server
    // Option containing the current page data?

    // App should let users:
    //      1. create characters for a setting
    //      2. join games hosted by other users to share character data and update a shared timeline
    //      3. host games as GM and shared details that are important to the setting when they need to
    //      4. create settings for players to play in
    //      5. be able to customize the appearance / layout of a character sheet
}

pub struct Game<'a> {
    game_id: u32,
    player_id: u32,
    active_view: ActiveView<'a>,
    game_data: GameData,
    gm: Option<GameMasterDataFile> // If this player can be a gm, then they have some gm data. If not, this is NONE
}
pub enum ActiveView<'a> {
    Character(&'a DataView<'a>),
    GameMaster(&'a DataView<'a>)
}
pub struct DataView<'a> {
    index: Index<'a>,
    shared_indexes: Vec<Index<'a>> // For characters that the player knows about, such as other players or NPCs
}

pub struct InstanceView<'a> {
    pub owner: &'a u32,
    pub inst: &'a MetaTypeInstance<'a>,
}

impl<'b> DataView<'b> {
    pub fn get_all_of_type(&self, t: &MetaType) -> Vec<InstanceView<'b>> {
        todo!()
    }

    pub fn get_owned_index(&self) -> &Index<'b> {
        return &self.index
    }

    pub fn get_id(&self) -> u32 {
        self.index.get_id()
    }
}

pub struct GameData {
    game_id: u32,
    owned_characters: Vec<CharacterDataFile>,
}

// TODO: Define DisplayIndex, which
// Maps a type to a DisplayData type
// A DisplayData holds what params are displayed
// A DisplayData also lets the user edit the data (if it is editable)
// Each field will have a "Is Mutable" flag

// TODO: Define a StyleIndex, which
// Which has a list of StyleSheets
// StyleSheets determine how a DisplayData is presented in HTML / CSS