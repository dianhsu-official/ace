mod account;
mod config;
mod generate;
mod language;
pub mod model;
mod parse;
mod submit;
use clap::Parser;

use self::account::AccountCommand;
use self::config::ConfigCommand;
use self::generate::GenerateCommand;
use self::language::LanguageCommand;
use self::model::Commands;
use self::parse::ParseCommand;
use self::submit::SubmitCommand;
use crate::context::CONTEXT;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
impl Cli {
    pub fn run() -> Result<(), String> {
        let cli = Cli::parse();
        match CONTEXT.lock() {
            Ok(mut context) => {
                context.workspace_directory = match std::env::current_dir() {
                    Ok(cur_dir) => match cur_dir.to_str() {
                        Some(dir_str) => Some(dir_str.to_string()),
                        None => None,
                    },
                    Err(_) => None,
                };
            }
            Err(_) => {}
        }
        let res = match cli.command {
            Commands::Account(args) => AccountCommand::handle(args),
            Commands::Config(args) => ConfigCommand::handle(args),
            Commands::Lang(args) => LanguageCommand::handle(args),
            Commands::Parse(args) => ParseCommand::handle(args),
            Commands::Gen(args) => GenerateCommand::handle(args),
            Commands::Submit(args) => SubmitCommand::handle(args),
        };
        match res {
            Ok(res) => {
                println!("{}", res);
            }
            Err(info) => {
                println!("{}", info);
            }
        }
        return Ok(());
    }
}
