use model::model::database::Database;



pub fn execute_command<T>(command: &str, db: &T) -> Result<String, String> 
    where T: Database
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
            info!("Uknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]))
        },
    }
}

fn help() -> Result<String, String> {
    info!("Command used: \"help\"");
    Ok(r#"Available commands:
    help                                           - Display this help message
    list [ruleset|setting|game|character]          - List all objects of specified type
    create ruleset <name>                          - Create a new ruleset
    create setting <name>                          - Create a new setting
    create game <name> <ruleset_id> <setting_id>   - Create a new game
    create character <name> <game_id> <owner_id>   - Create a new character
    get <id>                                       - Get details of an object
    update <type> <id> <field> <value>             - Update a field in an object
    delete <type> <id>                             - Delete an object
    exit                                           - Exit the program
    "#.to_string())
}