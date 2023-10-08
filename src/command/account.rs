use super::model::{AccountArgs, AccountOptions};
use crate::{constants::PLATFORM_MAP, utility::account::AccountUtility};
pub struct AccountCommand {}
impl AccountCommand {
    pub fn handle(args: AccountArgs) -> Result<String, String> {
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
            AccountOptions::Add => match AccountUtility::create_account(real_platform) {
                Ok(username) => {
                    return Ok(format!("Account {} added.", username));
                }
                Err(info) => {
                    return Err(info);
                }
            },
            AccountOptions::SetDefault => {
                let _ = match AccountUtility::choose_default_account(real_platform) {
                    Ok(_) => return Ok("Default account set.".to_string()),
                    Err(info) => {
                        return Err(info);
                    }
                };
            }
            AccountOptions::Update => {
                let _ = match AccountUtility::update_password(real_platform) {
                    Ok(_) => return Ok("Password updated.".to_string()),
                    Err(info) => {
                        return Err(info);
                    }
                };
            }
            AccountOptions::List => {
                let _ = match AccountUtility::get_account_list(real_platform) {
                    Ok(_) => return Ok("Account list printed.".to_string()),
                    Err(info) => {
                        return Err(info);
                    }
                };
            }
            AccountOptions::Remove => {
                let _ = match AccountUtility::remove_select_account(real_platform) {
                    Ok(_) => return Ok("Account removed.".to_string()),
                    Err(info) => {
                        return Err(info);
                    }
                };
            }
        }
    }
}
