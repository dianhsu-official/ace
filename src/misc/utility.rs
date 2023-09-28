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
    pub fn create_account(platform: &str) -> Result<String, String> {
        let mut username = String::new();
        while username.is_empty() {
            username = match Text::new("Enter your username: ").prompt() {
                Ok(username) => username,
                Err(_) => {
                    continue;
                }
            };
        }
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
    pub fn get_account_list(platform: &str) -> Result<(), String> {
        let accounts = CONFIG_DB.get_accounts(platform);
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Default"),
            Cell::new("Username"),
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
                Cell::new(local_time.format("%Y-%m-%d %H:%M:%S").to_string().as_str()),
            ]));
        }
        table.printstd();
        return Ok(());
    }
    pub fn remove_select_account(platform: &str) -> Result<(), String> {
        let accounts = CONFIG_DB.get_accounts(platform);
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
    pub fn choose_default_account(platform: &str) -> Result<(), String> {
        let accounts = CONFIG_DB.get_accounts(platform);
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
