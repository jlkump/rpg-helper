use rpg_helper::model::{core::Reference, database::{entity::{Entity, EntityID, StoreComponent}, Database, DatabaseEntity}, storable::types::Type, store::Store};

use crate::{commands::pretty_display_entity, data::{Context, EditorType, ProgramData}};

pub fn execute<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match parts[0]
    {
        "help" => help(),
        "back" => 
        {
            data.context = Context::Default;
            info!("[Type] Command Used: \"back\"");
            Ok("Returning to default".to_string())
        }
        "list" => 
        {
            if let Ok(e) = data.database.get_entity(&id)
            {
                if let Some(e) = e
                {
                    Ok(pretty_display_entity(e))
                }
                else
                {
                    warn!("[TypeStore] Self could not be found in database. ID {}", id);
                    Err(format!("Could not find entity of typestore {}", id))
                }
            }
            else
            {
                error!("[TypeStore] Database error while attempting to retrieve typestore entity {}", id);
                Err(format!("Database error"))
            }
        },
        "create" => create(id, parts, data),
        "edit" => edit(id, parts, data),
        _ =>
        {
            info!("[TypeStore] Unknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]))
        },
    }
}

fn help() -> Result<String, String>
{
    info!("[TypeStore] Command used: \"help\"");
    Ok(r#"
Available commands:
    help                                           - Display this help message
    create <NAME>                                  - Begin the creation of a type for the type store
    edit <NAME>                                    - Change an existing type
    delete <NAME>                                  - Deletes an entity
    list                                           - Lists all types in this type store
    save                                           - Saves the type store changes to disk
    back                                           - Exits the editor
    exit                                           - Exit the program
    "#.to_string())
}

fn create<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[TypeStore] Attempt to use \"create\" command, missing target for command");
        return Err("Command \"create\" is missing a target.".to_string());
    }

    let t = Type::new(parts[1]);
    data.context = Context::Editor(EditorType::Type(id, t));
    info!("[TypeStore] Command Used: \"create {}\"", parts[1]);
    Ok(format!("Started creation of type: {}", parts[1]))
}

fn edit<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[TypeStore] Attempt to use \"edit\" command, missing target for command");
        return Err("Command \"edit\" is missing a target.".to_string());
    }

    if let Ok(Some(Entity::Store(StoreComponent::TypeStore(ts)))) = data.database.get_entity(&id)
    {
        if let Ok(Some(t)) = ts.get(&Reference::new(ts.to_id().clone(), parts[1].to_string()))
        {
            data.context = Context::Editor(EditorType::Type(ts.to_id().clone(), t.clone().into_builder()));
            info!("[TypeStore] Command Used: \"edit {}\"", parts[1]);
            Ok(format!("Started editing of type: {}", parts[1]))
        }
        else
        {
            info!("[TypeStore] Attempt to edit non-existent type \"{}\"", parts[1]);
            Err(format!("Attempt to edit non-existent type: {}", parts[1]))
        }
    }
    else
    {
        warn!("[TypeStore] Self could not be found in database. ID {}", id);
        Err(format!("Could not find entity of typestore {}", id))
    }
}