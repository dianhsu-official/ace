use std::{env::current_dir, fs, str::FromStr};

use inquire::Select;

use crate::{
    constants::ProgramLanguage, context::CONTEXT, database::CONFIG_DB, snippet::Snippet,
    utility::Utility,
};

use super::model::GenerateArgs;

pub struct GenerateCommand {}

impl GenerateCommand {
    pub fn handle(args: GenerateArgs) -> Result<String, String> {
        if let Ok(dir) = current_dir() {
            if let Some(cur_path) = dir.to_str() {
                if let Ok(mut context) = CONTEXT.lock() {
                    context.update(cur_path);
                }
            }
        }
        let language = match args.language {
            Some(language) => language,
            None => match CONFIG_DB.get_config("default-language") {
                Ok(languge_str) => match ProgramLanguage::from_str(&languge_str) {
                    Ok(language) => language,
                    Err(_) => return Err("Can't convert default language".to_string()),
                },
                Err(_) => {
                    return Err("Default language not set, Run `ace lang set-default`".to_string());
                }
            },
        };
        let absolute_path = match current_dir() {
            Ok(dir) => dir,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let absolute_path_str = match absolute_path.to_str() {
            Some(path) => path,
            None => {
                return Err("Can't get current path".to_string());
            }
        };
        let workspace = match CONFIG_DB.get_config("workspace") {
            Ok(workspace) => workspace,
            Err(info) => {
                return Err(info);
            }
        };
        let (platform, _, _) =
            match Utility::get_identifiers_from_currrent_location(absolute_path_str, &workspace) {
                Ok(resp) => resp,
                Err(info) => {
                    return Err(info);
                }
            };
        let language_configs =
            match CONFIG_DB.get_language_config_by_language_and_platform(language, platform) {
                Ok(configs) => configs,
                Err(info) => {
                    return Err(info);
                }
            };
        let language_config = match language_configs.len() {
            0 => {
                return Err("Cannot find language config".to_string());
            }
            1 => language_configs[0].clone(),
            _ => match Select::new("Select language config", language_configs).prompt() {
                Ok(language_config) => language_config,
                Err(info) => {
                    return Err(info.to_string());
                }
            },
        };
        let filename = format!("code.{}", language_config.suffix);
        if let Ok(mut contest) = CONTEXT.lock() {
            contest.filename_with_extension = Some(filename.clone());
            contest.filename_without_extension = Some("code".to_string());
        }
        let mut content = String::new();
        if language_config.template_path != "" {
            let raw_content = match std::fs::read_to_string(language_config.template_path) {
                Ok(content) => content,
                Err(info) => {
                    log::info!(
                        "Read template file failed: {}, generate code with blank content.",
                        info
                    );
                    String::from("")
                }
            };
            content = match CONTEXT.lock() {
                Ok(context) => Snippet::replace(&context, &raw_content),
                Err(info) => {
                    return Err(info.to_string());
                }
            };
        }
        match fs::write(filename.clone(), content) {
            Ok(_) => {
                return Ok(format!("Generate {} success", filename));
            }
            Err(info) => {
                return Err(info.to_string());
            }
        };
    }
}
