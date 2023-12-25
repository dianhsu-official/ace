use inquire::MultiSelect;
use inquire::Select;
use prettytable::row;
use prettytable::Table;
use strum::IntoEnumIterator;

use super::model::LanguageArgs;
use crate::utility::language::LanguageUtility;
use crate::{command::model::LanguageOptions, constants::ProgramLanguage, database::CONFIG_DB};
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
            LanguageOptions::Add => LanguageUtility::prompt_to_add_languages(),
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
