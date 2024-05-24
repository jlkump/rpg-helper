use crate::data::indexes::Index;

use super::instance_view::InstanceView;

pub struct DataView<'a> {
    index: Index<'a>,
    shared_indexes: Vec<Index<'a>> // For characters that the player knows about, such as other players or NPCs
}

impl DataView<'_> {
    pub fn get_all_of_type(&self, t: &str) -> Vec<InstanceView> {
        todo!()
    }

    pub fn get_owned_index(&self) -> &Index {
        return &self.index
    }

    pub fn get_id(&self) -> u32 {
        self.index.get_id()
    }
}