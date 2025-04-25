#[macro_use] extern crate log;
extern crate simplelog;

use std::{fs::File, path::PathBuf};

use clap::{command, Parser, Subcommand};
use log::LevelFilter;
use rpg_helper::model::database::imp::sled::SledDB;
use repl::start_repl;
use simplelog::{Config, WriteLogger};

mod commands;
mod data;
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
    /// The database to use [sled]. Default to sled
    #[command(subcommand)]
    database: Option<Database>,
}

#[derive(Subcommand, Clone)]
enum Database
{
    /// Set the database config to be sled-db <PATH>
    SledDB
    {
        /// The location of the database to be opened on disk
        #[arg(short, long, value_name = "DATABASE_PATH")]
        path: Option<String>,
    },
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


    match cli.database.clone().unwrap_or(Database::SledDB { path: Some("./database/sled".to_string()) })
    {
        Database::SledDB { path } => 
        {
            let path = path.unwrap_or("./database/sled".to_string());
            start_repl(cli, SledDB::open(path).unwrap())
        },
    }
}