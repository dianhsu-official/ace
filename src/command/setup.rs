use crate::{database::CONFIG_DB, model::Platform, utility::language::LanguageUtility};

use super::model::SetupArgs;
use colored::Colorize;
use inquire::{min_length, Confirm, Password, PasswordDisplayMode, Select, Text};
use std::fmt::Display;
pub struct Workspace {
    pub path: String,
    pub name: String,
}
impl Display for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.path)
    }
}
pub struct SetupCommand;
impl SetupCommand {
    pub fn handle(_: SetupArgs) -> Result<String, String> {
        println!("Start setup...");

        match Confirm::new(
            "This setup will clear exist configurations, do you still want to continue?",
        )
        .prompt()
        {
            Ok(confirm) => {
                if !confirm {
                    return Ok("Setup cancelled.".to_string());
                }
            }
            Err(info) => {
                return Err(info.to_string());
            }
        };

        // Clear database
        println!("Clear database...");
        CONFIG_DB.drop_tables();
        CONFIG_DB.create_tables();
        // Setup workspace
        // 1. Current directory
        // 2. Home directory
        // 3. Input directory
        println!("Setup workspace...");
        let mut workspaces = Vec::new();
        match std::env::current_dir() {
            Ok(current_dir) => match current_dir.to_str() {
                Some(current_dir_str) => {
                    workspaces.push(Workspace {
                        path: current_dir_str.to_string(),
                        name: "Current directory".to_string(),
                    });
                }
                None => {
                    log::error!("Cannot convert current directory to string.");
                }
            },
            Err(_) => {
                log::error!("Cannot find current directory.");
            }
        }
        match std::env::var("HOME") {
            Ok(home_dir) => {
                workspaces.push(Workspace {
                    path: home_dir.clone(),
                    name: "Home directory".to_string(),
                });
            }
            Err(_) => {
                log::error!("Cannot find home directory.");
            }
        };
        workspaces.push(Workspace {
            path: "...".to_string(),
            name: "Input directory".to_string(),
        });
        let candidate_workspace = match Select::new("Choose a workspace", workspaces).prompt() {
            Ok(ans) => ans,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let workspace = match candidate_workspace.name.as_str() {
            "Input directory" => match Text::new("Enter the path: ").prompt() {
                Ok(path) => path,
                Err(_) => {
                    return Err("Path cannot be empty".to_string());
                }
            },
            _ => candidate_workspace.path.clone(),
        };
        match CONFIG_DB.create_config("workspace", &workspace) {
            Ok(_) => {
                println!("Workspace set to {}", workspace.green());
            }
            Err(info) => {
                return Err(info);
            }
        }

        // Setup account
        // 1. Codeforces
        // 2. AtCoder
        println!("Setup account...");
        loop {
            let platform = match Select::new(
                "Choose a platform",
                vec![Platform::Codeforces, Platform::AtCoder],
            )
            .prompt()
            {
                Ok(ans) => ans,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let username = match Text::new("Enter the username: ").prompt() {
                Ok(username) => username,
                Err(_) => {
                    return Err("Username cannot be empty".to_string());
                }
            };
            let password = match Password::new("Enter the password: ")
                .with_display_mode(PasswordDisplayMode::Masked)
                .with_formatter(&|password| "*".repeat(password.len()))
                .without_confirmation()
                .with_validator(min_length!(1, "Password cannot be empty"))
                .prompt()
            {
                Ok(password) => password,
                Err(_) => {
                    return Err("Password cannot be empty".to_string());
                }
            };
            match CONFIG_DB.add_account(platform, &username, &password) {
                Ok(_) => {
                    println!("Account {} added.", username.green());
                }
                Err(info) => {
                    return Err(info);
                }
            }

            let set_default = match Confirm::new("Set as default account?").prompt() {
                Ok(ans) => ans,
                Err(_) => false,
            };

            if set_default {
                match CONFIG_DB.set_default_account(platform, &username) {
                    Ok(_) => {
                        println!(
                            "Account {} set as default account on {}.",
                            username.green(),
                            platform.to_string().green()
                        );
                    }
                    Err(_) => {}
                }
            }
            match Confirm::new("Add more?").prompt() {
                Ok(ans) => {
                    if !ans {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            };
        }

        // Setup language
        println!("Setup language...");
        loop {
            match LanguageUtility::prompt_to_add_languages() {
                Ok(_) => {}
                Err(_) => {}
            }

            match Confirm::new("Add more?").prompt() {
                Ok(ans) => {
                    if !ans {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            };
        }

        println!("Setup finished.");
        Ok(String::new())
    }
}
