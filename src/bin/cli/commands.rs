use editor::execute_editor;
use rpg_helper::model::{database::{entity::{Entity, StoreComponent}, Database, DatabaseEntity, DatabaseMutator}, store::{types::TypeStore, Store}};

use crate::data::{Context, ProgramData};

mod editor;

pub fn execute_command<D: Database>(command: &str, data: &mut ProgramData<D>) -> Result<String, String> 
{
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty()
    {
        return Ok("".to_string());
    }
    
    match data.context.clone()
    {
        Context::Default => 
        {
            match parts[0]
            {
                "help" => help(),
                "create" => create(parts, data),
                "edit" => edit(parts, data),
                "delete" => todo!(),
                "list" => list(parts, data),
                _ => 
                {
                    info!("[Default] Unknown Command: \"{}\"", parts[0]);
                    Err(format!("Unknown command: {}", parts[0]))
                },
            }
        },
        Context::Editor(e) => execute_editor(e, parts, data),
    }
}

fn help() -> Result<String, String> 
{
    info!("[Default] Command used: \"help\"");
    Ok(r#"
Available commands:
    help                                           - Display this help message
    create [ruleset|setting|character|game|        - Create a container (ruleset, setting, etc.) 
        typestore|valuestore|etc]                 or store (typestore, valuestore, etc.) in memory to edit
    edit <EntityID>                                - Opens a CLI editor for the given entity
    delete <EntityID>                              - Deletes an entity
    list [ruleset|setting|typestore|etc]           - Lists all the Entities of a given type
    list <EntityID>                                - Lists all the values contained inside an entity
    exit                                           - Exit the program
    "#.to_string())
}

fn create<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() < 1
    {
        info!("[Default] Attempt to use \"create\" command, missing target for command");
        return Err("Command \"create\" is missing a target.".to_string());
    }

    match parts[1]
    {
        "typestore" | "TypeStore" | "Typestore" => 
        {
            match TypeStore::database_insert(&data.database, TypeStore::new())
            {
                Ok(id) =>
                {
                    info!("[Default] Command Used: \"create {}\"", parts[1]);
                    info!("[Default] Created Typestore with EntityID: {}", id);
                    Ok(format!("Created new TypeStore {}", id))
                },
                Err(e) => 
                {
                    warn!("[Default] Comand Failed: \"create {:?}\"", e);
                    error!("[Default] {:?}", e);
                    Err(format!("Failed to create typestore: {:?}", e))
                },
            }
        }
        _ => 
        {
            info!("[Default] Unknown \"create\" target: \"{}\"", parts[1]);
            Err(format!("Unknown \"create\" target: {}", parts[1]))
        },
    }
}

fn edit<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[Default] Attempt to use \"edit\" command, missing target for command");
        return Err("Command \"edit\" is missing a target. Supply the ID for the entity to edit.".to_string());
    }

    match uuid::Uuid::parse_str(parts[1])
    {
        Ok(id) =>
        {
            if let Ok(Some(context)) = data.get_editor_type_for_entity(&id) 
            {
                info!("[Default] Command Used: \"edit {}\"", parts[1]);
                data.context = Context::Editor(context.clone());
                Ok(format!("Begun editing {}", context))
            }
            else 
            {
                info!("[Default] Attempt to use \"edit\" command, invalid ID '{}'", parts[1]);
                Err(format!("ID '{}' for edit is not found in the database a database error occured.", parts[1]))
            }
        },
        Err(_) => 
        {
            info!("[Default] Attempt to use \"edit\" command, poorly formatted ID '{}'", parts[1]);
            Err(format!("ID '{}' for edit is not a UUID.", parts[1]))
        },
    }

}


fn list<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match uuid::Uuid::parse_str(parts[1])
    {
        Ok(id) => 
        {
            match data.database.get_entity(&id)
            {
                Ok(e) => 
                {
                    if let Some(e) = e
                    {
                        info!("[Default] Data of entity with ID '{}' displayed", id);
                        Ok(pretty_display_entity(e))
                    }
                    else
                    {
                        info!("[Default] Entity with ID '{}' not found in database", id);
                        Err(format!("Could not find entity {}", id))
                    }
                },
                Err(e) =>
                {
                    error!("[Default] Database error on reading entity with ID '{}': {:?}", id, e);
                    Err(format!("Database error: {:?}", e))
                },
            }
        },
        Err(_) =>
        {
            match parts[1]
            {

                "typestore" | "Typestore" | "TypeStore" => todo!(),
                _ => 
                {
                    info!("[Default] Unknown \"list\" target: \"{}\"", parts[1]);
                    Err(format!("Unknown \"list\" target: {}", parts[1]))
                },
            }
        },
    }
}

pub fn pretty_display_entity(e: Entity) -> String
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
                StoreComponent::TypeStore(type_store) => 
                {
                    let mut res = String::from(format!("Typestore {}\n", type_store.to_id()));
                    for t in type_store.get_all()
                    {
                        res.push_str(&format!("   {}\n", t));
                    }
                    res
                },
                StoreComponent::ValueStore() => todo!(),
                StoreComponent::WikiStore() => todo!(),
            }
        },
    }
}