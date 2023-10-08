mod account;
mod config;
mod generate;
mod language;
pub mod model;
mod parse;
mod submit;
mod test;
use self::account::AccountCommand;
use self::config::ConfigCommand;
use self::generate::GenerateCommand;
use self::language::LanguageCommand;
use self::model::Commands;
use self::parse::ParseCommand;
use self::submit::SubmitCommand;
use crate::context::CONTEXT;
use clap::Parser;
use colored::Colorize;
use log::LevelFilter;
use std::io::Write;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub verbose: bool,
}
impl Cli {
    pub fn run() -> Result<(), String> {
        let cli = Cli::parse();
        if cli.verbose {
            env_logger::builder()
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "[{} {}:{}] [{}] - {}",
                        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0),
                        record.level(),
                        record.args()
                    )
                })
                .filter(Some("ace"), LevelFilter::Info)
                .write_style(env_logger::WriteStyle::Auto)
                .init();
        } else {
            env_logger::builder()
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "[{} {}:{}] [{}] - {}",
                        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0),
                        record.level(),
                        record.args()
                    )
                })
                .filter(Some("ace"), LevelFilter::Warn)
                .write_style(env_logger::WriteStyle::Auto)
                .init();
        }
        match CONTEXT.lock() {
            Ok(mut context) => {
                context.current_directory = match std::env::current_dir() {
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
            Commands::Test(args) => test::TestCommand::handle(args),
        };
        match res {
            Ok(res) => {
                println!("{}", res.green());
            }
            Err(info) => {
                println!("{}", info.red());
            }
        }
        return Ok(());
    }
}
