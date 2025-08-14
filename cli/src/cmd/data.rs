use std::{fmt::Display, fs::File, io::{BufReader, BufWriter}, process::exit};

use colored::{ColoredString, Colorize};
use rpg_helper::api::{data::{context::Context, tag::Tag}, parse::json::ParseJson};

use crate::cmd::{default, CmdContext};

mod set;

#[derive(Clone, Debug)]
pub struct CtxData
{
    pub open: Option<Context>,
    pub dirty: bool,            // Dirty being true means we need to warn to save before close
    pub ctx_submode: CtxSubmode,
}

impl CtxData
{
    pub fn new() -> CtxData
    {
        CtxData { open: None, dirty: false, ctx_submode: CtxSubmode::Default }
    }

    pub fn to_prompt(&self) -> String
    {
        match &self.ctx_submode
        {
            CtxSubmode::Default =>
            {
                if self.open.is_some()
                {
                    "[Data - Open] >> ".to_string()
                }
                else
                {
                    "[Data] >> ".to_string()
                }
            },
            CtxSubmode::Set(ctx_data) => ctx_data.to_prompt(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CtxSubmode
{
    Default,
    Set(set::CtxData),
}

impl Display for CtxSubmode
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {   
            CtxSubmode::Default => write!(f, "Data"),
            CtxSubmode::Set(_) => write!(f, "Set"),
        }
    }
}

pub fn execute_command(s: &str, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString> 
{
    if let CmdContext::Data(ctx) = cmd_context
    {
        match &ctx.ctx_submode
        {
            CtxSubmode::Default =>
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
                    "mode" => 
                    {
                        if ctx.dirty && ctx.open.is_some()
                        {
                            warn!("[{}] \"mode {}\" used while dataset is open", cmd_context, parts[0]);
                            Err("A dataset is open. Please close before using \"mode\"".red())
                        }
                        else
                        {
                            default::mode_change(parts, cmd_context)
                        }
                    },
                    "open" => open(parts, ctx),
                    "new" => 
                    {
                        if ctx.open.is_some()
                        {
                            warn!("[{}] command \"new\" used while another dataset is open", cmd_context);
                            Err("Another dataset is open. Close it before creating a new one".red())
                        }
                        else
                        {
                            info!("[Data] Command used: \"new\"");
                            ctx.open = Some(Context::new());
                            Ok("Created new dataset".cyan())
                        }
                    },
                    "set" => set(parts, ctx),
                    "get" => get(parts, ctx, s),
                    "save" => save(parts, ctx),
                    "close" => close(parts, cmd_context),
                    _ => 
                    {
                        warn!("[Default] Unknown Command: \"{}\"", parts[0]);
                        Err(format!("Unknown command: {}", parts[0]).red())
                    },
                }
            },
            CtxSubmode::Set(_) => set::execute_command(s, cmd_context),
        }
    }
    else
    {
        error!("[Data] Tried to execute command in invalid context: \"{:?}\"", cmd_context);
        exit(1)
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
    open [json] <FILE_PATH>                        - Open a dataset from the given file
    new                                            - Creates a new open dataset
    set [a|c|e|m|t|x]                              - Opens mode [Data - Set] to set a value in the open dataset
    get [value|text] <TAG_NAME>                    - Get the value within the open dataset
    display <TAG_NAME>                             - Display all the data for the given tag
    remove [a|c|e|m|t|x] <TAG_NAME>                - Removes an attribute, conditional, equation, modifier, tag, 
                                                     or text with the given tag name.
    save [json] <FILE_PATH>                        - Saves the open dataset
    close                                          - Closes the open dataset
    "#.cyan())
}

fn open(parts: Vec<&str>, ctx_data: &mut CtxData) -> Result<ColoredString, ColoredString> 
{
    if ctx_data.open.is_some()
    {
        warn!("[Data - Open] Attempt to open dataset while another is still open");
        return Err("Another dataset is still open. Close before opening another.".red());
    }

    if parts.len() < 1
    {
        warn!("[Data] Attempt to use \"{}\" command, missing target for command", parts[0]);
        return Err("Command \"open\" is missing a target.".red());
    }

    match parts[1]
    {
        "json" => 
        {
            if parts.len() < 2
            {
                warn!("[Data] Attempt to use \"open json\" command, missing file path");
                return Err("Command \"open json\" is missing a file path.".red());
            }

            if let Ok(file) = File::open(parts[2])
            {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader)
                {
                    Ok(json) =>
                    {
                        match Context::from_json(json)
                        {
                            Ok(ctx) =>
                            {
                                info!("[Data] Command used: \"open json {}\"", parts[2]);
                                ctx_data.open = Some(ctx);
                                return Ok(format!("Opened dataset from \'{}\'", parts[2]).cyan());
                            },
                            Err(e) =>
                            {
                                error!("[Data] Attempt to use \"open json {}\" command, got api error:\n {:?}", parts[2], e);
                                return Err(format!("Could not parse file:\n {:?}", e).red());
                            },
                        }
                    },
                    Err(e) =>
                    {
                        error!("[Data] Attempt to use \"open json {}\" command, got serde error:\n {}", parts[2], e.to_string());
                        return Err(format!("Could not parse file:\n {}", e.to_string()).red());
                    },
                }
            }
            else
            {
                warn!("[Data] Attempt to use \"open json {}\" command, could not open file.", parts[2]);
                return Err(format!("Could not open file: {}", parts[2]).red());
            }
        },
        _ =>
        {
            warn!("[Data] Attempt to use \"open\" command, invalid target \"{}\" for command", parts[1]);
            return Err(format!("open target \"{}\" is unknown", parts[1]).red());
        }
    }
}

fn set(parts: Vec<&str>, ctx_data: &mut CtxData) -> Result<ColoredString, ColoredString>
{
    if ctx_data.open.is_none()
    {
        warn!("[Data] Attempt to \'set\' when no dataset is open");
        return Err("No dataset is open to edit.".red());
    }

    if parts.len() < 1
    {
        warn!("[Data] Attempt to use \"{}\" command, missing target for command", parts[0]);
        return Err("Command \"set\" is missing a target.".red());
    }

    match parts[1]
    {
        "t" | "tag" =>
        {
            ctx_data.ctx_submode = CtxSubmode::Set(set::CtxData::new_tag());
            info!("[Data] Command used: \"set {}\"", parts[1]);
            Ok("Entered mode [Set] for tag".cyan())
        },
        "a" | "attribute" =>
        {
            ctx_data.ctx_submode = CtxSubmode::Set(set::CtxData::new_attribute());
            info!("[Data] Command used: \"set {}\"", parts[1]);
            Ok("Entered mode [Set] for attribute".cyan())
        },
        "m" | "modifier" => 
        {
            ctx_data.ctx_submode = CtxSubmode::Set(set::CtxData::new_modifier());
            info!("[Data] Command used: \"set {}\"", parts[1]);
            Ok("Entered mode [Set] for modifier".cyan())
        },
        "e" | "equation" =>
        {
            ctx_data.ctx_submode = CtxSubmode::Set(set::CtxData::new_equation());
            info!("[Data] Command used: \"set {}\"", parts[1]);
            Ok("Entered mode [Set] for equation".cyan())
        },
        "c" | "conditional" =>
        {
            ctx_data.ctx_submode = CtxSubmode::Set(set::CtxData::new_conditional());
            info!("[Data] Command used: \"set {}\"", parts[1]);
            Ok("Entered mode [Set] for conditional".cyan())
        },
        "x" | "text" =>
        {
            ctx_data.ctx_submode = CtxSubmode::Set(set::CtxData::new_text());
            info!("[Data] Command used: \"set {}\"", parts[1]);
            Ok("Entered mode [Set] for text".cyan())
        },
        _ =>
        {
            warn!("[Data] Unrecognized option \"{}\" for \"{}\"", parts[1], parts[0]);
            Err(format!("Command \"set\" has unrecognized target \"{}\".", parts[1]).red())
        },
    }
}

fn get(parts: Vec<&str>, ctx_data: &mut CtxData, s: &str) -> Result<ColoredString, ColoredString>
{
    if ctx_data.open.is_none()
    {
        warn!("[Data] Attempt to get when no dataset is open");
        return Err("No dataset is open to read.".red());
    }

    if parts.len() < 1
    {
        warn!("[Data] Attempt to use \"{}\" command, missing target for command", parts[0]);
        return Err("Command \"get\" is missing a target.".red());
    }

    // Pre-process s such that the first two words are removed and the rest remains
    let s = s.splitn(3, ' ').nth(2).unwrap_or("");

    match parts[1]
    {
        "value" =>
        {
            if let Some(d) = &ctx_data.open
            {
                match Tag::from_str(s)
                {
                    Ok(t) =>
                    {
                        match d.get_value(&t)
                        {
                            Ok(v) =>
                            {
                                info!("[Data - Open] Command used: \"get value {}\"", t);
                                if let Some(v) = v
                                {
                                    Ok(format!("Got value: {}", v).cyan())
                                }
                                else
                                {
                                    Ok(format!("Found no value for \"{}\"", t).cyan())
                                }
                            },
                            Err(e) =>
                            {
                                error!("[Data - Open] Could not read value \"{}\":\n{:?}", t, e);
                                return Err(format!("Could not read value \"{}\":\n{:?}", t, e).red())
                            },
                        }

                    },
                    Err(e) =>
                    {
                        error!("[Data - Open] Parse error on input tag \"{}\":\n{:?}", s, e);
                        Err(format!("Could not parse given tag \"{}\":\n{:?}", s, e).red())
                    },
                }
            }
            else
            {
                warn!("[Data] Attempt to get when no dataset is open");
                return Err("No dataset is open to read.".red());
            }
        },
        "text" => todo!(),
        _ =>
        {
            warn!("[Data] Attempt to use \"get\" command, invalid target \"{}\" for command", parts[1]);
            return Err(format!("get target \"{}\" is unknown", parts[1]).red());
        }
    }
}

fn save(parts: Vec<&str>, ctx_data: &mut CtxData) -> Result<ColoredString, ColoredString>
{
    if ctx_data.open.is_none()
    {
        warn!("[Data] Attempt to save when no dataset is open");
        return Err("No dataset is open to save.".red());
    }

    if parts.len() < 1
    {
        warn!("[Data] Attempt to use \"{}\" command, missing target for command", parts[0]);
        return Err("Command \"save\" is missing a target.".red());
    }

    match parts[1]
    {
        "json" => 
        {
            if parts.len() < 2
            {
                warn!("[Data] Attempt to use \"save json\" command, missing file path");
                return Err("Command \"save json\" is missing a file path.".red());
            }

            if let Ok(file) = File::create(parts[2])
            {
                let writer = BufWriter::new(file);
                if let Some(ctx) = &ctx_data.open
                {
                    match serde_json::to_writer(writer, &ctx.to_json())
                    {
                        Ok(_) =>
                        {
                            ctx_data.dirty = false;
                            info!("[Data] Command used: \"save json {}\"", parts[2]);
                            return Ok(format!("Saved dataset to \'{}\'", parts[2]).cyan());
                        },
                        Err(e) =>
                        {
                            error!("[Data] command \"save json {}\" failed:\n{}", parts[2], e.to_string());
                            return Err(format!("Could not save file to: {}. Got error:\n{}", parts[2], e.to_string()).red());
                        },
                    }

                }
                else
                {
                    warn!("[Data] Attempt to save when no dataset is open");
                    return Err("No dataset is open to save.".red());
                }

            }
            else
            {
                error!("[Data] Attempt to use \"save json {}\" command, could not open file.", parts[2]);
                return Err(format!("Could not save file to: {}", parts[2]).red());
            }
        },
        _ =>
        {
            warn!("[Data] Attempt to use \"save\" command, invalid target \"{}\" for command", parts[1]);
            return Err(format!("save target \"{}\" is unknown", parts[1]).red());
        }
    }
}

fn close(parts: Vec<&str>, cmd_context: &mut CmdContext) -> Result<ColoredString, ColoredString>
{
    match &cmd_context
    {
        CmdContext::Default =>
        {
            error!("[{}] Tried to execute command in invalid context: \"{:?}\"", cmd_context, cmd_context);
            exit(1)
        },
        CmdContext::Data(ctx_data) =>
        {
            if ctx_data.dirty
            {
                if parts.len() < 1
                {
                    warn!("[{}] Attempt to close with unsaved data", cmd_context);
                    return Err(format!("Unsaved dataset. If you want to close without saving type \"close force\"").yellow());
                }
                else
                {
                    if parts[1].to_lowercase() != "force"
                    {
                        warn!("[{}] Attempt to close invalid confirmation \'{}\'", cmd_context, parts[1]);
                        return Err(format!("Unsaved dataset. If you want to close without saving type \"close force\"").yellow());
                    }
                    else
                    {
                        info!("[{}] Closing without saving...", cmd_context);
                    }
                }
            }
            *cmd_context = CmdContext::Data(CtxData::new());
            info!("[{}] Command used: \"close\"", cmd_context);
            Ok("Closed dataset".cyan())
        },
    }
}