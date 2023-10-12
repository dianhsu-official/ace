use std::{env::current_dir, fs, str::FromStr};

use crate::{constants::ProgramLanguage, context::CONTEXT, database::CONFIG_DB, snippet::Snippet};

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
            None => match CONFIG_DB.get_config("language") {
                Ok(languge_str) => match ProgramLanguage::from_str(&languge_str) {
                    Ok(language) => language,
                    Err(_) => return Err("Can't convert default language".to_string()),
                },
                Err(_) => {
                    return Err("Default language not set, Run `ace lang set-default`".to_string());
                }
            },
        };
        let language = match CONFIG_DB.get_language_config_by_language(language) {
            Ok(language) => language,
            Err(info) => {
                return Err(info);
            }
        };
        let filename = format!("code.{}", language.suffix);
        if let Ok(mut contest) = CONTEXT.lock() {
            contest.filename_with_extension = Some(filename.clone());
            contest.filename_without_extension = Some("code".to_string());
        }
        let mut content = String::new();
        if language.template_path != "" {
            let raw_content = match std::fs::read_to_string(language.template_path) {
                Ok(content) => content,
                Err(info) => {
                    return Err(info.to_string());
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
