use std::{collections::HashMap, sync::LazyLock};

use colored::{ColoredString, Colorize};
use rpg_helper::api::data::context::Context;

use crate::cmd::{default, CmdContext};

static mut DATA : LazyLock<HashMap<String, Context>> = LazyLock::new(|| HashMap::new());
static mut OPEN : Option<&Context> = None;

pub fn execute_command(s: &str, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString> 
{

    let parts: Vec<&str> = s.trim().split_whitespace().collect();
    if parts.is_empty()
    {
        return Ok("".clear());
    }

    debug!("Attempting execution of command {}", s);

    match parts[0]
    {
        "help" => help(),
        "mode" => default::mode_change(parts, cmd_context),
        "open" | "edit" | "close" => dataset(parts, cmd_context),
        _ => 
        {
            info!("[Default] Unknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]).red())
        },
    }
}

fn help() -> Result<ColoredString, ColoredString> 
{
    info!("[Data] Command used: \"help\"");
    Ok(r#"
Available commands (Data Mode):
    help                                           - Display this help message
    mode [default|data]                            - Create a container (ruleset, setting, etc.) 
                                                     or store (typestore, valuestore, etc.) in memory to edit
    open <DATASET_NAME>                            - Open a dataset with the given name (Creates if none exists)
    edit <DATASET_NAME>                            - Edit a dataset with the given name
    close <DATASET_NAME>                           - Close a dataset with the given name
    "#.cyan())
}

fn dataset(parts: Vec<&str>, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString> 
{
    if parts.len() < 1
    {
        info!("[Data] Attempt to use \"{}\" command, missing target for command", parts[0]);
        return Err("Command \"mode\" is missing a target.".red());
    }

    match parts[0]
    {
        "open" =>
        {
            if let Some(d) = DATA.get(parts[1])
            {
                OPEN = Some(d);
            }
            else
            {
                DATA.insert(parts[1], Context::new());
                OPEN = DATA.get(parts[1]);
            }
            Ok(format!("Opened {} dataset", parts[1]).cyan())
        }
    }
}