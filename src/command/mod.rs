mod account;
mod config;
use self::account::{AccountArgs, AccountCommand};
use self::config::ConfigCommand;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Manage account for ace, such as add, remove, list
    Account(AccountArgs),
    // Config
    Config,
}

impl Cli {
    pub fn run() -> Result<(), String> {
        let cli = Cli::parse();
        match cli.command {
            Commands::Account(args) => AccountCommand::handle(args),
            Commands::Config => ConfigCommand::handle(),
        }
    }
}
