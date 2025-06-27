#[macro_use] extern crate log;
extern crate simplelog;

use std::{fs::File, path::PathBuf};

use clap::{command, Parser, Subcommand};
use log::LevelFilter;
use rpg_helper::model::database::imp::sled::SledDB;
use repl::start_repl;
use simplelog::{Config, WriteLogger};

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