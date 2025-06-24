use rpg_helper::model::{database::{entity::{Entity, EntityID, StoreComponent}, Database}, store::Store};

use crate::data::{Context, EditorType, ProgramData};


pub fn execute<D: Database>(id: EntityID, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match parts[0]
    {
        "help" => help(),
        "display" => display(data),
        "name" => name(parts, data),
        "type" => ctype(parts, data),
        "list" => list(parts, data),
        "field" => field(parts, data),
        "enum" => cenum(parts, data),
        "back" => 
        {
            data.context = Context::Editor(EditorType::TypeStore(id));
            info!("[Type] Command Used: \"back\"");
            Ok("Returning to typestore editing".to_string())
        }
        "save" =>
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
                            info!("[Type] Command Used: \"save\"");
                            info!("[Type] Saved type \"{}\" for typestore '{}'", builder.name, id);
                            return Ok(format!("Saved type \"{}\" for typestore '{}'", builder.name, id));
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

fn help() -> Result<String, String>
{
    info!("[Type] Command used: \"help\"");
    Ok(r#"
Available commands:
    help                                           - Display this help message
    display                                        - Display the type as it is currently configured
    type [number|boolean|list|struct|enum]         - Set the type of the type
    name <NAME>                                    - Change the name
    list <LIST_TYPE>                               - Set the type of the list
    field set <FIELD_NAME> <FIELD_TYPE>            - Set a field for the Struct type
    field remove <FIELD_NAME>                      - Remove a field for the Struct type
    enum add <ENUM_NAME>                           - Add to the enum list
    enum remove <ENUM_NAME>                        - Remove given enum from the list
    save                                         - Complete creation and exit the editor
    back                                           - Discards the type and goes back to the type editor
    exit                                           - Exits the program
    "#.to_string())
}

fn display<D: Database>(data: &mut ProgramData<D>) -> Result<String, String>
{
    if let Context::Editor(EditorType::Type(id, builder)) = &mut data.context
    {
        info!("[Type] Command Used: \"display\" for type {} of typestore {}", builder.name, id);
        Ok(builder.get_pretty_string(3))
    }
    else
    {
        data.context = Context::Default;
        warn!("[Type] Invalid context state reached");
        Err("Invalid context state reached, returning to default state".to_string())
    }
}

fn name<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[Type] Attempt to use \"name\" command, missing name for command");
        return Err("Command \"name\" is missing a name.".to_string());
    }

    if let Context::Editor(EditorType::Type(_, builder)) = &mut data.context
    {
        builder.name = parts[1].to_string();
        info!("[Type] Command Used: \"name\" {}", parts[1]);
        Ok(format!("Name changed to {}", parts[1]))
    }
    else
    {
        data.context = Context::Default;
        warn!("[Type] Invalid context state reached");
        Err("Invalid context state reached, returning to default state".to_string())
    }
}

fn ctype<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[Type] Attempt to use \"type\" command, missing type for command");
        return Err("Command \"type\" is missing a type.".to_string());
    }

    if let Context::Editor(EditorType::Type(_, builder)) = &mut data.context
    {
        match parts[1]
        {
            "number" | "Number" => builder.as_number(),
            "boolean" | "Boolean" => builder.as_boolean(),
            "struct" | "Struct" => builder.as_struct(),
            "list" | "List" => builder.as_list(),
            "enum" | "Enum" => builder.as_enum(),
            _ => 
            {
                info!("[Type] Unknown target for command \"type\" {}", parts[1]);
                return Err(format!("Unknown target for command \"{}\"", parts[1]))
            }
        };
        info!("[Type] Command Used: \"type {}\"", parts[1]);
        info!("[Type] type '{}' modified to be '{}'", builder.name, parts[1]);
        Ok(format!("Type changed to {}", parts[1]))
    }
    else
    {
        data.context = Context::Default;
        warn!("[Type] Invalid context state reached");
        Err("Invalid context state reached, returning to default state".to_string())
    }
}

fn list<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[Type] Attempt to use \"list\" command, missing type for command");
        return Err("Command \"list\" is missing a type.".to_string());
    }

    if let Context::Editor(EditorType::Type(_, builder)) = &mut data.context
    {
        builder.as_list();
        builder.set_list_type(parts[1].to_string());
        info!("[Type] Command Used: \"type {}\"", parts[1]);
        info!("[Type] type '{}' modified to be 'List<{}>'", builder.name, parts[1]);
        Ok(format!("Type changed to List<{}>", parts[1]))
    }
    else
    {
        data.context = Context::Default;
        warn!("[Type] Invalid context state reached");
        Err("Invalid context state reached, returning to default state".to_string())
    }
}

fn field<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[Type] Attempt to use \"field\" command, missing subcommand for command");
        return Err("Command \"field\" is missing a subcommand [set|remove]".to_string());
    }

    if let Context::Editor(EditorType::Type(_, builder)) = &mut data.context
    {
        match parts[1]
        {
            "set" => 
            {
                if parts.len() <= 3
                {
                    info!("[Type] Attempt to use \"field set\" command, missing field name and/or field type");
                    return Err("Command \"field set\" is missing field name and/or field type".to_string());
                }

                builder.as_struct();
                builder.set_struct_field(parts[2].to_string(), parts[3].to_string());
                info!("[Type] Command Used: \"field set {} {}\"", parts[2], parts[3]);
                info!("[Type] type '{}' modified to have '{}: {}'", builder.name, parts[2], parts[3]);
                Ok(format!("field '{}' set to type '{}'", parts[2], parts[3]))
            },
            "remove" =>
            {
                if parts.len() <= 2
                {
                    info!("[Type] Attempt to use \"field remove\" command, missing field name");
                    return Err("Command \"field remove\" is missing field name".to_string());
                }

                builder.as_struct();
                builder.remove_struct_field(parts[2]);
                info!("[Type] Command Used: \"field remove {}\"", parts[2]);
                info!("[Type] type '{}' removed field '{}'", builder.name, parts[2]);
                Ok(format!("field '{}' removed", parts[2]))
            },
            _ => 
            {
                info!("[Type] Unknown \"field\" Subcommand: \"{}\"", parts[1]);
                Err(format!("Unknown \"field\" Subcommand: {}", parts[1]))
            }
        }
    }
    else
    {
        data.context = Context::Default;
        warn!("[Type] Invalid context state reached");
        Err("Invalid context state reached, returning to default state".to_string())
    }
}

fn cenum<D: Database>(parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    if parts.len() <= 1
    {
        info!("[Type] Attempt to use \"enum\" command, missing subcommand for command");
        return Err("Command \"enum\" is missing a subcommand [add|remove]".to_string());
    }

    if let Context::Editor(EditorType::Type(_, builder)) = &mut data.context
    {
        match parts[1]
        {
            "add" => 
            {
                if parts.len() <= 2
                {
                    info!("[Type] Attempt to use \"enum add\" command, missing name");
                    return Err("Command \"enum add\" is missing name".to_string());
                }

                builder.as_enum();
                builder.add_to_enums(parts[2].to_string());
                info!("[Type] Command Used: \"enum add {}\"", parts[2]);
                info!("[Type] type '{}' added '{}' to enums", builder.name, parts[2]);
                Ok(format!("'{}' added to enums", parts[2]))
            },
            "remove" =>
            {
                if parts.len() <= 2
                {
                    info!("[Type] Attempt to use \"enum remove\" command, missing name");
                    return Err("Command \"enum remove\" is missing name".to_string());
                }

                builder.as_enum();
                builder.remove_from_enums(parts[2].to_string());
                info!("[Type] Command Used: \"enum remove {}\"", parts[2]);
                info!("[Type] type '{}' removed enum '{}'", builder.name, parts[2]);
                Ok(format!("enum '{}' removed", parts[2]))
            },
            _ => 
            {
                info!("[Type] Unknown \"enum\" Subcommand: \"{}\"", parts[1]);
                Err(format!("Unknown \"enum\" Subcommand: {}", parts[1]))
            }
        }
    }
    else
    {
        data.context = Context::Default;
        warn!("[Type] Invalid context state reached");
        Err("Invalid context state reached, returning to default state".to_string())
    }
}