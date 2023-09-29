use crate::config::Platform;
use crate::config::PLATFORMS;
use crate::misc::database::CONFIG_DB;
use chrono::DateTime;
use inquire::min_length;
use inquire::MultiSelect;
use inquire::Password;
use inquire::PasswordDisplayMode;
use inquire::Text;
use prettytable::{Cell, Row, Table};
pub struct Utility {}
impl Utility {
    fn get_platform_from_cmd(platform_from_cmd: Option<Platform>) -> Result<Platform, String> {
        match platform_from_cmd {
            Some(platform) => Ok(platform),
            None => {
                let platform_vec = PLATFORMS.clone();
                match inquire::Select::new("Choose a platform", platform_vec).prompt() {
                    Ok(ans) => Ok(ans),
                    Err(_) => {
                        return Err("Error when choosing a platform".to_string());
                    }
                }
            }
        }
    }
    pub fn create_account(platform_from_cmd: Option<Platform>) -> Result<String, String> {
        let platform = match Self::get_platform_from_cmd(platform_from_cmd) {
            Ok(platform) => platform,
            Err(info) => {
                return Err(info);
            }
        };
        let username = match Text::new("Enter your username: ").prompt() {
            Ok(username) => username,
            Err(_) => {
                return Err("Username cannot be empty".to_string());
            }
        };
        let password = match Password::new("Enter your password: ")
            .with_display_mode(PasswordDisplayMode::Masked)
            .with_formatter(&|password| "*".repeat(password.len()))
            .with_validator(min_length!(1, "Password cannot be empty"))
            .prompt()
        {
            Ok(password) => password,
            Err(_) => {
                return Err("Password cannot be empty".to_string());
            }
        };
        match CONFIG_DB.add_account(platform, &username, &password) {
            Ok(_) => {}
            Err(info) => {
                return Err(info);
            }
        }
        return Ok(username);
    }
    pub fn get_account_list(platform_from_cmd: Option<Platform>) -> Result<(), String> {
        let accounts = CONFIG_DB.get_accounts(platform_from_cmd);
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Default"),
            Cell::new("Username"),
            Cell::new("Platform"),
            Cell::new("Last use (local time)"),
        ]));
        for account in accounts {
            let default_val = match account[3].as_str() {
                "1" => "*",
                _ => "",
            };
            let last_use = match DateTime::parse_from_rfc3339(&account[4]) {
                Ok(last_use) => last_use,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let local_time = last_use.with_timezone(&chrono::Local);
            table.add_row(Row::new(vec![
                Cell::new(default_val),
                Cell::new(&account[0]),
                Cell::new(&account[5]),
                Cell::new(local_time.format("%Y-%m-%d %H:%M:%S").to_string().as_str()),
            ]));
        }
        table.printstd();
        return Ok(());
    }
    pub fn remove_select_account(platform_from_cmd: Option<Platform>) -> Result<(), String> {
        let platform = match Self::get_platform_from_cmd(platform_from_cmd) {
            Ok(platform) => platform,
            Err(info) => {
                return Err(info);
            }
        };
        let accounts = CONFIG_DB.get_accounts(Some(platform));
        let mut options: Vec<String> = Vec::new();
        for account in accounts {
            options.push(format!("{}", account[0]));
        }
        if options.is_empty() {
            return Err(String::from("No account"));
        }
        match MultiSelect::new("Choose accounts to remove: ", options).prompt() {
            Ok(ans) => {
                CONFIG_DB.remove_accounts(platform, ans);
            }
            Err(_) => {
                return Err(String::from("Error when choosing accounts"));
            }
        };
        return Ok(());
    }
    pub fn choose_default_account(platform_from_cmd: Option<Platform>) -> Result<(), String> {
        let platform = match Self::get_platform_from_cmd(platform_from_cmd) {
            Ok(platform) => platform,
            Err(info) => {
                return Err(info);
            }
        };
        let accounts = CONFIG_DB.get_accounts(Some(platform));
        let mut options: Vec<String> = Vec::new();
        for account in accounts {
            options.push(format!("{}", account[0]));
        }
        if options.is_empty() {
            return Err(String::from("No account"));
        }
        let _ = match inquire::Select::new("Choose an account", options).prompt() {
            Ok(ans) => ans,
            Err(_) => {
                return Err(String::from("Error when choosing an account"));
            }
        };
        return Ok(());
    }
}
