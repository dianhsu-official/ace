use colored::Colorize;
use inquire::MultiSelect;
use inquire::{Select, Text};
use prettytable::row;
use prettytable::Table;
use strum::IntoEnumIterator;

use super::model::LanguageArgs;
use crate::platform::OnlineJudge;
use crate::{
    command::model::LanguageOptions, constants::ProgramLanguage, database::CONFIG_DB,
    model::Platform,
};
pub struct LanguageCommand {}

impl LanguageCommand {
    pub fn handle(args: LanguageArgs) -> Result<String, String> {
        match args.options {
            LanguageOptions::List => {
                let mut table = Table::new();
                table.add_row(row![
                    "alias",
                    "suffix",
                    "platform",
                    "identifier",
                    "submit_id",
                    "submit_description",
                    "template_path",
                    "compile_command",
                    "execute_command",
                    "clear_command"
                ]);
                let language_configs = match CONFIG_DB.get_language_config() {
                    Ok(configs) => configs,
                    Err(_) => {
                        return Err(String::from("List language config failed"));
                    }
                };
                for item in language_configs {
                    table.add_row(row![
                        item.alias,
                        item.suffix,
                        item.platform,
                        item.identifier,
                        item.submit_id,
                        item.submit_description,
                        item.template_path,
                        item.compile_command,
                        item.execute_command,
                        item.clear_command
                    ]);
                }
                table.printstd();
                return Ok("".to_string());
            }
            LanguageOptions::Add => {
                let languages: Vec<ProgramLanguage> = ProgramLanguage::iter().collect::<Vec<_>>();
                let language_identifier = match Select::new(
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
                let platform = match Select::new(
                    "Choose target platform to set language:",
                    Platform::iter().collect::<Vec<_>>(),
                )
                .prompt()
                {
                    Ok(ans) => ans,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                let template_path_prompt_message = format!(
                    "Enter the template path(e.g. {}):",
                    "C:\\Users\\dianhsu\\template\\any_fileame.cpp".bright_green()
                );
                let template_path = match Text::new(&template_path_prompt_message).prompt() {
                    Ok(value) => value,
                    Err(_) => String::from(""),
                };
                let compile_command_prompt_message = format!(
                    "Enter the compile command(e.g. {}):",
                    "g++ -std=c++17 -O2 -Wall %$full$% -o a.out".bright_green()
                );
                let compile_command = match Text::new(&compile_command_prompt_message).prompt() {
                    Ok(value) => value,
                    Err(_) => String::from(""),
                };
                let execute_command_prompt_message = format!(
                    "Enter the execute command(e.g. {}):",
                    "./a.out".bright_green()
                );
                let execute_command = match Text::new(&execute_command_prompt_message).prompt() {
                    Ok(value) => value,
                    Err(_) => String::from(""),
                };
                let clear_command_prompt_message = format!(
                    "Enter the clear command(e.g. {}):",
                    "rm a.out".bright_green()
                );
                let clear_command = match Text::new(&clear_command_prompt_message).prompt() {
                    Ok(value) => value,
                    Err(_) => String::from(""),
                };
                let alias = match Text::new("Enter the alias: ").prompt() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err("Alias cannot be empty".to_string());
                    }
                };
                let submit_language_infos = OnlineJudge::get_platform_languages(platform);
                let filtered_submit_language_infos = submit_language_infos
                    .iter()
                    .filter(|info| info.language == language_identifier)
                    .collect::<Vec<_>>();
                let submit_languge_info = match Select::new(
                    "Select exact language to submit code:",
                    filtered_submit_language_infos,
                )
                .prompt()
                {
                    Ok(submit_language_info) => submit_language_info,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                match CONFIG_DB.add_lang_config(
                    &alias,
                    &suffix,
                    platform,
                    language_identifier,
                    &submit_languge_info.id,
                    &submit_languge_info.description,
                    &template_path,
                    &compile_command,
                    &execute_command,
                    &clear_command,
                ) {
                    Ok(_) => Ok(String::from("Set language config success")),
                    Err(info) => Err(info),
                }
            }
            LanguageOptions::Delete => {
                let language_configs = match CONFIG_DB.get_language_config() {
                    Ok(configs) => configs,
                    Err(_) => {
                        return Err(String::from("List language config failed"));
                    }
                };

                let remove_configs =
                    match MultiSelect::new("Select language config to remove:", language_configs)
                        .prompt()
                    {
                        Ok(remove_configs) => remove_configs,
                        Err(info) => {
                            return Err(info.to_string());
                        }
                    };
                let remove_ids = remove_configs
                    .into_iter()
                    .map(|config| config.id)
                    .collect::<Vec<_>>();
                match CONFIG_DB.remove_lang_config(remove_ids) {
                    Ok(_) => {}
                    Err(info) => {
                        return Err(info);
                    }
                }
                return Ok("Success to remove language config".to_string());
            }
            LanguageOptions::SetDefault => {
                let default_lang = match Select::new(
                    "Select default language:",
                    ProgramLanguage::iter().collect::<Vec<_>>(),
                )
                .prompt()
                {
                    Ok(default_lang) => default_lang,
                    Err(_) => {
                        return Err("Select default language failed".to_string());
                    }
                };
                match CONFIG_DB.set_config("default-language", default_lang.to_string().as_str()) {
                    Ok(_) => {
                        return Ok("Set default language success".to_string());
                    }
                    Err(info) => {
                        return Err(info);
                    }
                }
            }
        }
    }
}
