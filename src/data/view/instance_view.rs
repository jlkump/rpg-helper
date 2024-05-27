use crate::data::meta_type::MetaTypeInstance;

pub struct InstanceView<'a> {
    pub owner: &'a u32,
    pub inst: &'a MetaTypeInstance<'a>,
}