use editor::execute_editor;
use model::model::{database::{Database, DatabaseEntity, DatabaseEntityBuilder, DatabaseID}, store::types::{TypeStore, TypeStoreBuilder}};

use crate::repl::{Context, ProgramData};

mod editor;

pub fn execute_command<D: Database>(command: &str, data: &mut ProgramData, db: &D) -> Result<String, String> 
{
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty()
    {
        return Ok("".to_string());
    }
    
    let con = data.context.clone(); 
    match con
    {
        Context::Default => 
        {
            match parts[0]
            {
                "help" => help(),
                "create" => create(parts, data, db),
                "edit" => edit(parts, data, db),
                "delete" => todo!(),
                "list" => todo!(),
                _ => 
                {
                    info!("[Default] Unknown Command: \"{}\"", parts[0]);
                    Err(format!("Unknown command: {}", parts[0]))
                },
            }
        },
        Context::Editor(e) => execute_editor(e, parts, data, db),
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
    exit                                           - Exit the program
    "#.to_string())
}

fn create<D: Database>(parts: Vec<&str>, data: &mut ProgramData, db: &D) -> Result<String, String>
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
            let type_store = TypeStore::debug_new();
            let id = type_store.to_id().clone();
            data.type_stores.insert(id.clone(), type_store);
            info!("[Default] Command Used: \"create {}\"", parts[1]);
            info!("[Default] Created Typestore with EntityID: {}", id);
            Ok(format!("Created new TypeStore {}", id))
        }
        _ => 
        {
            info!("[Default] Unknown \"create\" target: \"{}\"", parts[1]);
            Err(format!("Unknown \"create\" target: {}", parts[1]))
        },
    }
}

fn edit<D: Database>(parts: Vec<&str>, data: &mut ProgramData, db: &D) -> Result<String, String>
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
            if let Some(context) = data.get_editor_type_for_entity(&id) 
            {
                info!("[Default] Command Used: \"edit {}\"", parts[1]);
                data.context = Context::Editor(context.clone());
                Ok(format!("Begun editing {}", context))
            }
            else 
            {
                info!("[Default] Attempt to use \"edit\" command, invalid ID '{}'", parts[1]);
                Err(format!("ID '{}' for edit is not found in the database or in memory.", parts[1]))
            }
        },
        Err(_) => 
        {
            info!("[Default] Attempt to use \"edit\" command, poorly formatted ID '{}'", parts[1]);
            Err(format!("ID '{}' for edit is not a UUID.", parts[1]))
        },
    }

}