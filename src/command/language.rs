use inquire::{Select, Text};
use strum::IntoEnumIterator;

use super::model::LanguageArgs;
use crate::{command::model::LanguageOptions, constants::ProgramLanguage, database::CONFIG_DB};
pub struct LanguageCommand {}

impl LanguageCommand {
    pub fn handle(args: LanguageArgs) -> Result<String, String> {
        match args.options {
            LanguageOptions::List => match CONFIG_DB.list_lang_config() {
                Ok(_) => {
                    return Ok(String::from("List language config success"));
                }
                Err(_) => {
                    return Err(String::from("List language config failed"));
                }
            },
            LanguageOptions::Set => {
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
                match CONFIG_DB.set_lang_config(
                    lang,
                    &suffix,
                    &template_path,
                    &compile_command,
                    &execute_command,
                    &clear_command,
                ) {
                    Ok(_) => {}
                    Err(info) => return Err(info),
                };
                let set_default = match Select::new(
                    "Do you want to set this language as default?",
                    vec!["Yes", "No"],
                )
                .prompt()
                {
                    Ok(ans) => ans,
                    Err(_) => {
                        return Err("Set default failed".to_string());
                    }
                };
                if set_default == "Yes" {
                    match CONFIG_DB.set_config("language", lang.to_string().as_str()) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                return Ok(String::from("Set language config success"));
            }
        }
    }
}
