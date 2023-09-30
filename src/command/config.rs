use crate::{constants::ProgramLanguage, database::CONFIG_DB};
use inquire::{MultiSelect, Select, Text};
use strum::IntoEnumIterator;

use super::model::{ConfigArgs, ConfigOptions};

pub struct ConfigCommand {}

impl ConfigCommand {
    pub fn handle(args: ConfigArgs) -> Result<String, String> {
        match args.options {
            ConfigOptions::Get => {
                let config_list = CONFIG_DB
                    .list_config()
                    .into_iter()
                    .map(|x| x[0].clone())
                    .collect::<Vec<String>>();
                let key = match Select::new("Choose a config", config_list).prompt() {
                    Ok(ans) => ans,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                match CONFIG_DB.get_config(&key) {
                    Ok(value) => return Ok(format!("{}: {}", key, value)),
                    Err(info) => {
                        return Err(info);
                    }
                }
            }
            ConfigOptions::Add => {
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
            ConfigOptions::List => {
                let config_list = CONFIG_DB.list_config();
                let mut table = prettytable::Table::new();
                table.add_row(prettytable::Row::new(vec![
                    prettytable::Cell::new("Key"),
                    prettytable::Cell::new("Value"),
                ]));
                for config in config_list {
                    table.add_row(prettytable::Row::new(vec![
                        prettytable::Cell::new(&config[0]),
                        prettytable::Cell::new(&config[1]),
                    ]));
                }
                table.printstd();
                Ok(String::new())
            }
            ConfigOptions::Set => {
                let config_list = CONFIG_DB
                    .list_config()
                    .into_iter()
                    .map(|x| x[0].clone())
                    .collect::<Vec<String>>();
                let key = match Select::new("Choose a config", config_list).prompt() {
                    Ok(ans) => ans,
                    Err(info) => {
                        return Err(info.to_string());
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
            ConfigOptions::Lang => {
                let languages: Vec<ProgramLanguage> = ProgramLanguage::iter().collect::<Vec<_>>();
                let lang = match Select::new(
                    "Choose a language to set compile and execute command:",
                    languages,
                )
                .prompt()
                {
                    Ok(ans) => ans,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                println!("{:?}", lang);
                let suffix = match Text::new("Enter the suffix (e.g. [cpp, cxx, py]): ").prompt() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err("Suffix cannot be empty".to_string());
                    }
                };
                let template_path =
                    match Text::new("Enter the template path (allow empty): ").prompt() {
                        Ok(value) => value,
                        Err(_) => String::from(""),
                    };
                let compile_command =
                    match Text::new("Enter the compile command (allow empty): ").prompt() {
                        Ok(value) => value,
                        Err(_) => String::from(""),
                    };
                let execute_command = match Text::new("Enter the execute command: ").prompt() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err("Execute command cannot be empty".to_string());
                    }
                };
                let clear_command =
                    match Text::new("Enter the clear command (allow empty): ").prompt() {
                        Ok(value) => value,
                        Err(_) => String::from(""),
                    };
                return match CONFIG_DB.set_lang_config(
                    lang,
                    &suffix,
                    &template_path,
                    &compile_command,
                    &execute_command,
                    &clear_command,
                ) {
                    Ok(_) => Ok(format!("Language {} set.", lang.to_string())),
                    Err(info) => Err(info),
                };
            }
            ConfigOptions::Remove => {
                let config_list = CONFIG_DB
                    .list_config()
                    .into_iter()
                    .map(|x| x[0].clone())
                    .collect::<Vec<String>>();
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
