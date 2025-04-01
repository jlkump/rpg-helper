#[macro_use] extern crate log;
extern crate simplelog;

use std::{fs::File, path::PathBuf};

use clap::{command, Parser};
use log::LevelFilter;
use model::model::database::Database;
use repl::start_repl;
use simplelog::{Config, WriteLogger};

mod commands;
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
    // #[arg(short, long)]
    // remote: bool,
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

    let mut test = TestDB {};
    start_repl(cli, &mut test)
}

struct TestDB
{

}

impl Database for TestDB
{
    fn insert_entity(&mut self, e: model::model::database::entity::Entity) -> Result<(), model::model::database::DatabaseError> {
        todo!()
    }

    fn get_entity(&self, id: &model::model::database::entity::EntityID) -> Result<model::model::database::entity::Entity, model::model::database::DatabaseError> {
        todo!()
    }

    fn modify_entity(&mut self, id: &model::model::database::entity::EntityID, n: model::model::database::entity::Entity) -> Result<model::model::database::entity::Entity, model::model::database::DatabaseError> {
        todo!()
    }

    fn remove_entity(&mut self, id: &model::model::database::entity::EntityID) -> Result<model::model::database::entity::Entity, model::model::database::DatabaseError> {
        todo!()
    }
}