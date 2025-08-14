#[macro_use] extern crate log;
extern crate simplelog;

use std::{fs::File, path::PathBuf};

use clap::{command, Parser, Subcommand};
use log::LevelFilter;
use simplelog::{Config, WriteLogger};

mod cmd;
mod repl;

#[derive(Parser)]
#[command(name = "RPGHelper-CLI")]
#[command(version = "0.1")]
#[command(about = "A command line interface for interacting directly with the RPG Helper server database", long_about = None)]
#[command(author = "Landon Kump")]
struct Cli 
{
    /// Set the log file. If not specified, logging file is set to cli.log by default.
    #[arg(short, long, value_name = "FILE")]
    log: Option<PathBuf>,
    /// The log level, ranging from 0 to 5. By default, is set to 5.
    #[arg(long, value_name = "LOG_LEVEL")]
    log_level: Option<u8>,
    /// The IP address of the remote server
    #[arg(short, long, value_name = "REMOTE")]
    remote: Option<PathBuf>,
    // /// The type of cli interface to run
    // #[command(subcommand)]
    // iterface: Option<Interface>,
}


const DEFAULT_LOG_LEVEL: u8 = 3;

fn main() -> std::io::Result<()>
{
    let cli = Cli::parse();
    let file_name = cli.log.clone().unwrap_or("cli.log".into());
    let log_level = match cli.log_level.unwrap_or(DEFAULT_LOG_LEVEL)
    {
        5 => LevelFilter::Trace,
        4 => LevelFilter::Debug,
        3 => LevelFilter::Info,
        2 => LevelFilter::Warn,
        1 => LevelFilter::Error,
        0 | _ => LevelFilter::Off,
    };
    WriteLogger::init(log_level, Config::default(), File::create(file_name).unwrap()).unwrap();
    info!("Log initialized successfully with log level {}", cli.log_level.unwrap_or(DEFAULT_LOG_LEVEL));
    
    repl::start_repl(cli);
    Ok(())
}