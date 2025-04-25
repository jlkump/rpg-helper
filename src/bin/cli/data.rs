use rpg_helper::model::{core::Error, database::{entity::{Entity, EntityID, StoreComponent}, Database, DatabaseEntity}, storable::types::TypeBuilder, store::types::TypeStore};

/// Global data that is in-memory for the execution of the program
#[derive(Debug, Clone)]
pub struct ProgramData<D: Database>
{
    pub context: Context,
    pub database: D,
}

impl<D: Database> ProgramData<D>
{
    pub fn new(database: D) -> ProgramData<D>
    {
        ProgramData { context: Context::Default, database }
    }
}

impl<D: Database> ProgramData<D>
{
    pub fn get_editor_type_for_entity(&self, id: &EntityID) -> Result<Option<EditorType>, Error>
    {
        if let Some(e) = self.database.get_entity(id)?
        {
            match e
            {
                Entity::Database(database_record) => todo!(),
                Entity::User(user) => todo!(),
                Entity::Container(container_component) => todo!(),
                Entity::Store(store_component) => 
                {
                    match store_component
                    {
                        StoreComponent::EquationStore() => todo!(),
                        StoreComponent::EventStore() => todo!(),
                        StoreComponent::LocationStore() => todo!(),
                        StoreComponent::MapStore() => todo!(),
                        StoreComponent::TypeStore(type_store) => Ok(Some(EditorType::TypeStore(type_store.to_id().clone()))),
                        StoreComponent::ValueStore() => todo!(),
                        StoreComponent::WikiStore() => todo!(),
                    }
                },
            }
        }
        else
        {
            Ok(None)
        }
    }
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
    TypeStore(EntityID),
    Type(EntityID, TypeBuilder)
}

impl EditorType
{
    pub fn get_name(&self) -> String
    {
        match &self
        {
            EditorType::TypeStore(_) => "TypeStore".to_owned(),
            EditorType::Type(_, type_builder) => type_builder.name.to_owned(),
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
            EditorType::Type(id, type_builder) => write!(f, "Type ({}) for Typestore ({})", type_builder.name.to_owned(), id),
        }
    }
}