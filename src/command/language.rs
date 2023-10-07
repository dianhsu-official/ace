use inquire::{Select, Text};
use strum::IntoEnumIterator;

use super::model::LanguageArgs;
use crate::{
    command::model::LanguageOptions, constants::ProgramLanguage, database::CONFIG_DB,
    model::Platform, platform::atcoder::AtCoder, platform::codeforces::Codeforces,
    traits::OnlineJudge,
};
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
                    "Choose a platform to set language:",
                    Platform::iter().collect::<Vec<_>>(),
                )
                .prompt()
                {
                    Ok(ans) => ans,
                    Err(info) => {
                        return Err(info.to_string());
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
                    Err(_) => String::from(""),
                };
                let clear_command =
                    match Text::new("Enter the clear command (allow empty): ").prompt() {
                        Ok(value) => value,
                        Err(_) => String::from(""),
                    };
                let alias = match Text::new("Enter the alias: ").prompt() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err("Alias cannot be empty".to_string());
                    }
                };
                let submit_language_infos = match match platform {
                    Platform::AtCoder => AtCoder::new().unwrap().get_submit_languages(),
                    Platform::Codeforces => Codeforces::new().unwrap().get_submit_languages(),
                } {
                    Ok(submit_language_infos) => submit_language_infos,
                    Err(info) => {
                        return Err(info);
                    }
                };
                let submit_languge_info = match Select::new(
                    "Select exact language to submit code:",
                    submit_language_infos,
                )
                .prompt()
                {
                    Ok(submit_language_info) => submit_language_info,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                match CONFIG_DB.set_lang_config(
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
                    match CONFIG_DB.set_config("language", language_identifier.to_string().as_str())
                    {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                return Ok(String::from("Set language config success"));
            }
        }
    }
}
