use model::model::{database::{Database, DatabaseEntity, DatabaseEntityBuilder, DatabaseID}, store::types::{TypeStore, TypeStoreBuilder}};

use crate::repl::ProgramData;



pub fn execute_command<D: Database>(command: &str, data: &mut ProgramData, db: &D) -> Result<String, String> 
{
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Ok("".to_string());
    }
    
    match parts[0] {
        "help" => help(),
        // "list" => list_objects(parts.get(1).copied(), db),
        // "create" => create_object(parts, db),
        // "get" => get_object(parts, db),
        // "update" => update_object(parts, db),
        // "delete" => delete_object(parts, db),
        _ => 
        {
            info!("Unknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]))
        },
    }
}

fn help() -> Result<String, String> {
    info!("Command used: \"help\"");
    Ok(r#"Available commands:
    help                                           - Display this help message
    create [ruleset|setting|character|game|        - Create a container (ruleset, setting, etc.) 
           typestore|valuestore|etc]                 or store (typestore, valuestore, etc.) in memory to edit
    edit <EntityID>                                - Opens a CLI editor for the given entity
    delete <EntityID>                              - Deletes an entity
    exit                                           - Exit the program
    "#.to_string())
}

fn create<D: Database>(parts: Vec<&str>, data: &mut ProgramData, db: &D) -> Result<String, String>
{
    if parts.len() < 1
    {
        return Err("Command \"create\" is missing a target.".to_string());
    }

    match parts[1]
    {
        "typestore" | "TypeStore" | "Typestore" => 
        {
            let type_store = TypeStore::debug_new();
            Ok(format!("Created new TypeStore {}", type_store.to_id()))
        }
        _ => 
        {
            info!("Unknown \"create\" target: \"{}\"", parts[1]);
            Err(format!("Unknown \"create\" target: {}", parts[1]))
        },
    }
}