// The character context will hold all the information of the active character. This is used
// just for the character viewer.

// TODO: first define a basic Ars Magica character in ./data to use for testing
// Also define a basic game and game display in ./data as well for displaying that character's data

use crate::data::character_data::CharacterData;

pub struct CharacterView<'a> {
    pub data: CharacterData<'a>, // The data in-memory. When modified, must also be saved to disk to be persistent
}

impl CharacterView<'_> {
    pub fn save() {
        // Saves any changes made to the character to disk
        todo!()
    }
}

pub(crate) fn use_character<'a>() -> Option<CharacterData<'a>> {
    Some(CharacterData::default())
}