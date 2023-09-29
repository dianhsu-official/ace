use inquire::{MultiSelect, Select, Text};

use crate::database::CONFIG_DB;

use super::model::{ConfigArgs, ConfigOptions};

pub struct ConfigCommand {}

impl ConfigCommand {
    pub fn handle(args: ConfigArgs) -> Result<String, String> {
        match args.options {
            ConfigOptions::Get => {
                let config_list = CONFIG_DB.list_config();
                let key = match Select::new("Choose a config", config_list).prompt() {
                    Ok(ans) => ans,
                    Err(_) => {
                        return Err("Error when choosing a config".to_string());
                    }
                };
                match CONFIG_DB.get_config(&key) {
                    Ok(value) => return Ok(format!("{}: {}", key, value)),
                    Err(info) => {
                        return Err(info);
                    }
                }
            }
            ConfigOptions::Create => {
                let key = match Text::new("Enter the name: ").prompt() {
                    Ok(key) => key,
                    Err(_) => {
                        return Err("Key cannot be empty".to_string());
                    }
                };
                let value = match Text::new("Enter the value: ").prompt() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err("Value cannot be empty".to_string());
                    }
                };
                match CONFIG_DB.create_config(&key, &value) {
                    Ok(_) => Ok(format!("Config {} created.", key)),
                    Err(info) => Err(info),
                }
            }
            ConfigOptions::Set => {
                let config_list = CONFIG_DB.list_config();
                let key = match Select::new("Choose a config", config_list).prompt() {
                    Ok(ans) => ans,
                    Err(_) => {
                        return Err("Error when choosing a config".to_string());
                    }
                };
                let value = match Text::new("Enter the value: ").prompt() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err("Value cannot be empty".to_string());
                    }
                };
                match CONFIG_DB.set_config(&key, &value) {
                    Ok(_) => Ok(format!("Config {} set.", key)),
                    Err(info) => Err(info),
                }
            }
            ConfigOptions::Remove => {
                let config_list = CONFIG_DB.list_config();
                let keys = match MultiSelect::new("Choose configs to remove", config_list).prompt()
                {
                    Ok(ans) => ans,
                    Err(_) => {
                        return Err("Error when choosing configs".to_string());
                    }
                };
                match CONFIG_DB.remove_config(keys) {
                    Ok(_) => Ok(format!("Config removed.")),
                    Err(info) => Err(info),
                }
            }
        }
    }
}
