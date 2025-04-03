use std::collections::HashMap;

use model::model::{database::{entity::EntityID, Database}, store::types::TypeStore};
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{commands::execute_command, Cli};

/// Global data that is in-memory for the execution of the program
pub struct ProgramData
{
    pub context: Context,
    pub type_stores: HashMap<EntityID, TypeStore>,
}

/// A context is used to determine the response to commands
/// For example, in the Editor context, an entity is the subject of
/// the editor and can be added to, removed from, and updated.
pub enum Context
{
    Default,
    Editor(EntityID),
}

impl ProgramData
{
    fn new() -> ProgramData
    {
        ProgramData { type_stores: HashMap::new(), context: Context::Default }
    }
}

pub fn start_repl<T>(cli: Cli, db: &mut T) -> std::io::Result<()>
    where T: Database
{   
    println!("\
        ┌──────────────────────┐\n\
        │  RPG Helper CLI Tool │\n\
        └──────────────────────┘\n\
        Welcome to the RPG Helper CLI Tool. This tool allows for\n\
        manipulation of the database behind the RPG Helper server directly.\n\
        \n\
        If this program is run remotely, the \"remote\" flag must be specified\n\
        with the ip address of the server and an authorization token.\n\
        \n\
        Type \"help\" for a list of commands. Type \"exit\" or \"quit\" to close the cli.\n\
    ");
    info!("CLI session started");

    let mut rl = DefaultEditor::new().unwrap();
    let mut data = ProgramData::new();

    loop 
    {
        let prompt = match data.context
        {
            Context::Default => ">> ",
            Context::Editor(uuid) => "[Editor] >> ",
        };
        let readline = rl.readline(prompt);
        match readline 
        {
            Ok(line) => 
            {
                rl.add_history_entry(line.as_str()).unwrap();
                
                if line.trim() == "exit" || line.trim() == "quit" 
                {
                    info!("Command used: \"{}\"", line.trim());
                    info!("CLI session ended");
                    println!("Ending the session");
                    break;
                }
                
                debug!("Attempting execution of command {}", line.as_str());
                match execute_command(line.as_str(), &mut data, db)
                {
                    Ok(output) => print!("{}", output),
                    Err(e) => println!("[Error]: {}", e),
                }
            },
            Err(ReadlineError::Interrupted) => 
            {
                info!("Signal Interrupt (CTRL-C)");
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => 
            {
                info!("Signal EOF (CTRL-D)");
                println!("CTRL-D");
                break;
            },
            Err(err) => 
            {
                error!("{:?}", err);
                println!("[Error]: {:?}", err);
                break;
            }
        }
    }
    
    Ok(())
}