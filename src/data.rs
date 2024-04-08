use std::{any::type_name, collections::HashMap};

use self::meta_type::{FieldValue, MetaType, MetaTypeInstance};

pub mod meta_type;
pub mod equation;
pub mod timeline;

pub struct TypeIndex {
    types: Vec<MetaType>
}

impl TypeIndex {
    pub fn get_type(&self, type_name: &String) -> Option<&MetaType> {
        self.types.iter().find(|mt| {mt.get_name().eq(type_name)})
    }
}

pub struct DataIndex<'a> {
    data: HashMap<String, MetaTypeInstance<'a>>,
    modifiers: Vec<Modifier>
}

impl DataIndex<'_> {
    pub fn get_value(&self, name: &String, used_for: Option<&String>) -> i32 {
        if let Some(mti) = self.data.get(name) {
            if let Some(use_case) = used_for {
                if let Some(m) = self.modifiers.iter().find(|modifier| modifier.target.eq(name)) {
                    return MetaTypeInstance::get_value(self, mti) + m.apply_modifier(self, use_case)
                }
            }
            return MetaTypeInstance::get_value(self, mti)
        }
        return 0
    }
}

pub struct Modifier {
    name: String,
    target: String,
    val_name: String,
    source: String,
    use_case: ModifierUseCase
}

impl Modifier {
    fn apply_modifier(&self, data: &DataIndex, use_case: &String) -> i32 {
        match &self.use_case {
            ModifierUseCase::Never => 0,
            ModifierUseCase::Always => data.get_value(&self.val_name, Some(&self.name)),
            ModifierUseCase::OnMatch(s) => if s.eq(use_case) {
                data.get_value(&self.val_name, Some(&self.name))
            } else {
                0
            }
        }
    }
}

pub enum ModifierUseCase {
    Never,
    Always,
    OnMatch(String)
}

pub struct CharacterData<'a> {
    types: TypeIndex,
    data: DataIndex<'a>
}

impl CharacterData<'_> {

}