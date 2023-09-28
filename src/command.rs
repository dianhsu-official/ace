use clap::{Args, Parser, Subcommand};

use crate::config::{ATCODER, CODEFORCES};

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
#[derive(Args)]
pub struct ParseArgs {
    /// Choose a platform
    #[arg(short, long)]
    platform: String,
    identifier: String,
}

#[derive(Args)]
pub struct AccountArgs {
    /// Set the platform
    #[arg(short, long)]
    platform: String,
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();
        match cli.command {
            Commands::Account(args) => {
                let platform = args.platform;
                if CODEFORCES.contains(&platform.as_str()) {
                    println!("Codeforces");
                } else if ATCODER.contains(&platform.as_str()) {
                    println!("Atcoder");
                } else {
                    println!("Unknown platform");
                }
            }
            Commands::Config => {
                println!("Config");
            }
        }
    }
}
