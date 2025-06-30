use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{cmd::{data, default, CmdContext}, Cli};

use colored::Colorize;

pub fn start_repl(cli: Cli) -> std::io::Result<()>
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
    let mut cmd_context = CmdContext::Default;

    loop
    {
        let prompt = match &cmd_context
        {
            CmdContext::Default => "[Default] >> ",
            CmdContext::Data => "[Data] >> ",
        };

        let readline = rl.readline(&prompt);
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
                
                let res = match cmd_context
                {
                    CmdContext::Default => default::execute_command(line.as_str(), &mut cmd_context),
                    CmdContext::Data => data::execute_command(line.as_str(), &mut cmd_context),
                };

                match res
                {
                    Ok(output) => println!("{}", output),
                    Err(e) => println!("{}{}","[Error]: ".red(), e),
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