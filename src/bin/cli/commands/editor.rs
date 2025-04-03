use rpg_helper::model::{database::{entity::{Entity, EntityID, StoreComponent}, Database}, storable::types::Type, store::Store};

use crate::data::{Context, EditorType, ProgramData};

use super::pretty_display_entity;

pub fn execute_editor<D: Database>(editor_type: EditorType, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match editor_type
    {
        EditorType::TypeStore(id) => execute_typestore(id, parts, data),
        EditorType::Type(id, _) => execute_type(id, parts, data),
    }
}

fn execute_typestore<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match parts[0]
    {
        "help" => typestore_help(),
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
        "create" => typestore_create(id, parts, data),
        _ =>
        {
            info!("[TypeStore] Unknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]))
        },
    }
}

fn typestore_help() -> Result<String, String>
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

fn typestore_create<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[TypeStore] Attempt to use \"create\" command, missing target for command");
        return Err("Command \"create\" is missing a target.".to_string());
    }

    let t = Type::new(parts[1]);
    data.context = Context::Editor(EditorType::Type(id, t));
    info!("[TypeStore] Command Used: create {}", parts[1]);
    Ok(format!("Started creation of type: {}", parts[1]))
}

fn execute_type<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match parts[0]
    {
        "help" => type_help(),
        "type" => todo!(),
        "name" => todo!(),
        "back" => 
        {
            data.context = Context::Editor(EditorType::TypeStore(id));
            info!("[Type] Command Used: \"back\"");
            Ok("Returning to typestore editing".to_string())

        }
        "create" =>
        {
            if let Context::Editor(EditorType::Type(id, builder)) = data.context.clone()
            {
                if let Ok(Some(Entity::Store(StoreComponent::TypeStore(mut ts)))) = data.database.get_entity(&id)
                {
                    if let Ok(_) = ts.set(builder.clone())
                    {
                        if let Ok(_) = data.database.update_entity(&id, ts.into())
                        {
                            data.context = Context::Editor(EditorType::TypeStore(id));
                            info!("[Type] Command Used: \"create\"");
                            info!("[Type] Created type \"{}\" for typestore '{}'", builder.name, id);
                            return Ok(format!("Created type \"{}\" for typestore '{}'", builder.name, id));
                        }
                    }
                }
            }
            Err("Error occured when creating type".to_string())
        },
        _ =>
        {
            info!("[Type] Unknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]))
        },
    }
}

fn type_help() -> Result<String, String>
{
    info!("[Type] Command used: \"help\"");
    Ok(r#"
Available commands:
    help                                           - Display this help message
    display                                        - Display the type as it is currently configured
    type [Number|Boolean|List|Struct|Eum]          - Set the type of the type
    name <NAME>                                    - Change the name
    list <LIST_TYPE>                               - Set the type of the list
    field <FIELD_NAME> <FIELD_TYPE>                - Create a field for the Struct type
    create                                         - Complete creation and exit the editor
    back                                           - Discards the type and goes back to the type editor
    exit                                           - Exits the program
    "#.to_string())
}
