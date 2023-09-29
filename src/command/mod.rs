mod account;
mod config;
pub mod model;
use clap::Parser;

use self::account::AccountCommand;
use self::config::ConfigCommand;
use self::model::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
impl Cli {
    pub fn run() -> Result<(), String> {
        let cli = Cli::parse();
        let _ = match cli.command {
            Commands::Account(args) => AccountCommand::handle(args),
            Commands::Config(args) => ConfigCommand::handle(args),
        };

        return Ok(());
    }
}
