use std::sync::Arc;

use super::{indexes::Index, meta_type::MetaTypeInstance, view::data_view::DataView};

// Character data is an abstracted way of viewing the data contained in a data view
pub struct CharacterData<'a> {
    data: Index<'a>
}

impl<'b> CharacterData<'b> {
    pub fn default<'a>() -> CharacterData<'a> {
        todo!()
    }

    pub fn get_all_of_type(&self, t: &str) -> &MetaTypeInstance<'b> {
        todo!()
    }
}