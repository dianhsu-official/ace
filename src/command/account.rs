use crate::config::PLATFORM_MAP;
use crate::misc::utility::Utility;
use clap::{Args, Subcommand};
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
    #[command(subcommand)]
    pub options: AccountOptions,
    #[arg(short, long)]
    pub platform: Option<String>,
}
pub struct AccountCommand {}
impl AccountCommand {
    pub fn handle(args: AccountArgs) -> Result<(), String> {
        let platform = args.platform;
        let real_platform = match platform {
            Some(platform) => match PLATFORM_MAP.get(platform.as_str()) {
                Some(platform) => Some(*platform),
                None => {
                    return Err(format!("Platform {} not found", platform));
                }
            },
            None => None,
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
}
