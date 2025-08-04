use colored::{ColoredString, Colorize};

use crate::cmd::{data::CtxData, CmdContext};

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
        "mode" => mode_change(parts, cmd_context),
        _ => 
        {
            warn!("[Default] Unknown Command: \"{}\"", parts[0]);
            Err(format!("Unknown command: {}", parts[0]).red())
        },
    }
}

fn help() -> Result<ColoredString, ColoredString> 
{
    info!("[Default] Command used: \"help\"");
    Ok(r#"
Available commands:
    help                                           - Display this help message
    mode [default|data]                            - Create a container (ruleset, setting, etc.) 
                                                     or store (typestore, valuestore, etc.) in memory to edit
    "#.cyan())
}

pub fn mode_change(parts: Vec<&str>, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString> 
{
    if parts.len() < 1
    {
        warn!("[{}] Attempt to use \"mode\" command, missing target for command", cmd_context);
        return Err("Command \"mode\" is missing a target.".red());
    }

    match parts[1]
    {
        "default" =>
        {
            info!("[{}] Command used: \"mode default\"", cmd_context);
            *cmd_context = CmdContext::Default;
            Ok("Entered default mode".cyan())
        },
        "data" =>
        {
            info!("[{}] Command used: \"mode data\"", cmd_context);
            *cmd_context = CmdContext::Data(CtxData::new());
            Ok("Entered data mode".cyan())
        },
        _ =>
        {
            warn!("[{}] Attempt to use \"mode\" command, invalid target \"{}\" for command", cmd_context, parts[1]);
            return Err(format!("mode target \"{}\" is unknown", parts[1]).red());
        }
    }
}