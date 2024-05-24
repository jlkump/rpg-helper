// The character context will hold all the information of the active character. This is used
// just for the character viewer.

// TODO: first define a basic Ars Magica character in ./data to use for testing
// Also define a basic game and game display in ./data as well for displaying that character's data

use crate::data::character_data::CharacterData;

pub(crate) fn use_character() -> Option<CharacterData> {
    Some(CharacterData::default())
}