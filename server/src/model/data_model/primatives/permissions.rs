use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::data_model::storage::{location::{LocationRef, MapRef}, timeline::EventTypeRef, types::TypeRef, values::ValueRef, wiki::WikiPageRef};

// This might be better placed in storage module?

pub type PlayerId = uuid::Uuid;
pub type CharacterId = uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct GamePermissions {
    // wiki_permissions: HashMap<String, Vec<Permission>>,      // Permission to see a wiki page in a game
    // type_permissions: HashMap<String, Vec<Permission>>,      // Permission to use a type in a game
    // character_permissions: HashMap<String, Vec<Permission>>, // Permission to see a character in a game. Should index by character ID

    // Rework
    players: Vec<uuid::Uuid>,      // Who is invited an allowed to play in the game
    game_masters: Vec<uuid::Uuid>, // Who can be game master
    active_gm: uuid::Uuid,         // Who is the active game master (allow multiple? Should be ok to, but will check later)
}

impl GamePermissions {
    pub fn get_wikipage_permissions(&self, wiki_page: &WikiPageRef) -> Vec<Permission> {
        todo!()
    }

    pub fn get_type_permissions(&self, t_ref: &TypeRef) -> Vec<Permission> {
        todo!()
    }

    pub fn get_value_permissions(&self, v_ref: &ValueRef) -> Vec<Permission> {
        todo!()
    }

    pub fn get_location_permissions(&self, loc_ref: &LocationRef) -> Vec<Permission> {
        todo!()
    }

    pub fn get_map_permissions(&self, map_ref: &MapRef) -> Vec<Permission> {
        todo!()
    }

    pub fn get_event_permissions(&self, e_ref: &EventTypeRef) -> Vec<Permission> {
        todo!()
    }

    pub fn get_character_permissions(&self, id: &CharacterId) -> Vec<Permission> {
        todo!()
    }
}


#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub enum PermissionSetting {
    OnlyOwner,
    OnlyGameMaster, 
    OwnerAndGameMaster,
    // The OnlyGameMasters setting means that the user with this permission only have it if they are the active GM.
    AllPlayers,
    Custom(CustomPermissionSetting)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct CustomPermissionSetting {
    characters: Vec<CharacterId>,
    players: Vec<PlayerId>,
}

// Might need a way to go from user_id to Permissions
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Permission {
    owner: PlayerId,
    read_permissions: PermissionSetting,
    write_permissions: PermissionSetting,
}
