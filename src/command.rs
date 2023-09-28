use crate::{
    config::{Platform, PLATFORM_MAP},
    misc::utility::Utility,
};
use clap::{Args, Parser, Subcommand};
use inquire::Select;

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

#[derive(Subcommand)]
pub enum AccountOptions {
    Add,
    List,
    ChooseDefault,
    UpdatePassword,
    Remove,
}
#[derive(Args)]
pub struct AccountArgs {
    /// Set the platform
    platform: String,
    #[command(subcommand)]
    options: AccountOptions,
}

impl Cli {
    pub fn run() -> Result<(), String> {
        let cli = Cli::parse();
        match cli.command {
            Commands::Account(args) => {
                let platform = args.platform.as_str();
                let real_platform = match PLATFORM_MAP.get(&platform) {
                    Some(real_platform) => match real_platform {
                        Platform::Codeforces => "codeforces",
                        Platform::Atcoder => "atcoder",
                    },
                    None => {
                        return Err("Unknown platform".to_string());
                    }
                };
                match args.options {
                    AccountOptions::Add => match Utility::create_account(real_platform) {
                        Ok(username) => {
                            println!("Account {} added.", username);
                        }
                        Err(info) => {
                            return Err(info);
                        }
                    },
                    AccountOptions::ChooseDefault => {
                        let _ = match Utility::choose_default_account(real_platform) {
                            Ok(_) => {}
                            Err(info) => {
                                return Err(info);
                            }
                        };
                    }
                    AccountOptions::UpdatePassword => {
                        println!("Update password");
                    }
                    AccountOptions::List => {
                        let _ = match Utility::get_account_list(real_platform) {
                            Ok(_) => {}
                            Err(info) => {
                                return Err(info);
                            }
                        };
                    }
                    AccountOptions::Remove => {
                        let _ = match Utility::remove_select_account(real_platform) {
                            Ok(_) => {}
                            Err(info) => {
                                return Err(info);
                            }
                        };
                    }
                }
                return Ok(());
            }
            Commands::Config => {
                let options = vec![Platform::Codeforces, Platform::Atcoder];
                let ans = Select::new("Select an option", options).prompt();
                match ans {
                    Ok(ans) => println!("{}", ans),
                    Err(_) => {
                        return Err("Error when choosing option".to_string());
                    }
                }
                return Ok(());
            }
        }
    }
}
