// The character context will hold all the information of the active character. This is used
// just for the character viewer.

// TODO: first define a basic Ars Magica character in ./data to use for testing
// Also define a basic game and game display in ./data as well for displaying that character's data

use yew::{hook, use_state};
use yew::UseStateHandle;

use crate::data::character_data::CharacterData;

#[derive(PartialEq)]
pub struct CharacterView<'a> {
    pub data: CharacterData<'a>, // The data in-memory. When modified, must also be saved to disk to be persistent
}

impl CharacterView<'_> {
    pub fn save() {
        // Saves any changes made to the character to disk if the user owns the character
        todo!()
    }
}

#[hook]
pub(crate) fn use_character() {

    // let character = use_state(|| Some(CharacterData::default()));
    // character
    todo!()
}