use model::model::database::{entity::EntityID, Database};

use crate::repl::{EditorType, ProgramData};



pub fn execute_editor<D: Database>(editor_type: EditorType, parts: Vec<&str>, data: &mut ProgramData, db: &D) -> Result<String, String>
{
    match editor_type
    {
        EditorType::TypeStore(id) => execute_typestore(id, parts, data, db),
    }
}

fn execute_typestore<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData, db: &D) -> Result<String, String>
{
    match parts[0]
    {
        "help" => typestore_help(),
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

fn typestore_create(id: EntityID, parts: Vec<&str>, data: &mut ProgramData, db: &D) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[TypeStore] Attempt to use \"create\" command, missing target for command");
        return Err("Command \"create\" is missing a target.".to_string());
    }

    
}