use std::collections::HashMap;

use model::model::{database::entity::EntityID, store::types::TypeStore};

/// Global data that is in-memory for the execution of the program
#[derive(Debug, Clone)]
pub struct ProgramData
{
    pub context: Context,
    pub type_stores: HashMap<EntityID, TypeStore>,
}

impl ProgramData
{
    pub fn get_editor_type_for_entity(&self, id: &EntityID) -> Option<EditorType>
    {
        if self.type_stores.contains_key(id)
        {
            Some(EditorType::TypeStore(id.clone()))
        }
        else
        {
            None
        }
    }
}

pub struct MemoryDatabase
{

}

/// A context is used to determine the response to commands
/// For example, in the Editor context, an entity is the subject of
/// the editor and can be added to, removed from, and updated.
#[derive(Debug, Clone)]
pub enum Context
{
    Default,
    Editor(EditorType),
}

#[derive(Debug, Clone)]
pub enum EditorType
{
    TypeStore(EntityID)
}

impl EditorType
{
    pub fn get_name(&self) -> String
    {
        match &self
        {
            EditorType::TypeStore(_) => "TypeStore".to_owned(),
        }
    }
}

impl std::fmt::Display for EditorType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            EditorType::TypeStore(uuid) => write!(f, "TypeStore ({})", uuid),
        }
    }
}

impl ProgramData
{
    fn new() -> ProgramData
    {
        ProgramData { type_stores: HashMap::new(), context: Context::Default }
    }
}