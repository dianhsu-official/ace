use std::{env::current_dir, path::Path, process};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    directory: Option<String>,
}
#[derive(Subcommand)]
pub enum Commands {
    /// Parse contest from platform
    Parse(ParseArgs),
    /// Manage account
    Account(AccountArgs),
    // Config
    Config
}
#[derive(Args)]
pub struct ParseArgs {
    /// Choose a platform
    #[arg(short, long)]
    platform: String,
    identifier: String,
}

#[derive(Args)]
pub struct AccountArgs {
    #[arg(short, long)]
    platform: String,
}

impl Cli {
    #[allow(unused_variables)]
    pub fn run() {
        let cli = Cli::parse();
        
    }
}
