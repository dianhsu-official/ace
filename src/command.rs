use std::{env::current_dir, path::Path, process};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::{config::Config, tool::Tool};
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
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Platform {
    /// https://codeforces.com
    Cf,
}
#[derive(Args)]
pub struct ParseArgs {
    /// Choose a platform
    #[arg(short, long)]
    platform: Platform,
    identifier: String,
}

#[derive(Args)]
pub struct AccountArgs {
    #[arg(short, long)]
    platform: Platform,
}

impl Cli {
    #[allow(unused_variables)]
    pub fn run(config: &mut Config, config_path: &Path) {
        let cur = match current_dir() {
            Ok(path) => path,
            Err(err) => {
                log::error!("Get work dir failed. {}", err);
                process::exit(1);
            }
        };
        let problems_config = cur.as_path().join("config.json");
        let cli = Cli::parse();
        match &cli.command {
            Commands::Parse(args) => {
                let platform_name = match args.platform {
                    Platform::Cf => "Codeforces",
                };
                println!("{}, {}", platform_name, args.identifier);
            }
            Commands::Account(args) => match args.platform {
                Platform::Cf => {
                    let mut idx = -1;
                    while idx != 0 {
                        println!("Choose your operation: ");
                        println!("0) Quit");
                        println!("1) Show accounts");
                        println!("2) Add account");
                        println!("3) Remove account");
                        println!("4) Set default account.");
                        println!("5) Check accounts.");

                        idx = Tool::choose_index(6);
                        match idx {
                            0 => {}
                            1 => config.cf.list_accont(),
                            2 => match config.cf.add_account() {
                                Ok(account) => {
                                    println!("Add account {} successed.", &account.handle);
                                    config.cf.accounts.push(account);
                                    match config.save(config_path) {
                                        Ok(_) => {}
                                        Err(_) => {}
                                    }
                                }
                                Err(err) => {
                                    println!("{}", err);
                                }
                            },
                            3 => {
                                config.cf.list_accont();
                                let rdx = Tool::choose_index(
                                    config.cf.accounts.len().try_into().unwrap(),
                                );
                                let account = config.cf.accounts.remove(rdx.try_into().unwrap());
                                println!("Account \"{}\" removed.", account.handle);
                            }
                            4 => {
                                config.cf.list_accont();
                                let rdx = Tool::choose_index(
                                    config.cf.accounts.len().try_into().unwrap(),
                                );
                                let account = config.cf.accounts.remove(rdx.try_into().unwrap());
                                println!("Set account \"{}\" as default.", account.handle);
                                config.cf.accounts.insert(0, account);
                            }
                            5 => config.cf.check_accounts(),
                            _ => println!("Index out of range."),
                        }
                    }
                }
            },
        }
    }
}
