use inquire::Select;
use prettytable::{Table, row};

use super::ConfigDatabase;
use crate::constants::ProgramLanguage;
use crate::model::{LanguageConfig, LanguageSubmitConfig, Platform};
use std::str::FromStr;
impl ConfigDatabase {
    pub fn get_language_submit_config_from_suffix(
        &self,
        suffix: &str,
    ) -> Result<Vec<LanguageSubmitConfig>, String> {
        let query = String::from("SELECT alias, suffix, platform, identifier, submit_id, submit_description FROM language WHERE suffix = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        if let Err(info) = stmt.bind((1, suffix)) {
            return Err(info.to_string());
        }
        let mut vec = Vec::new();
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let alias = row.read::<&str, _>("alias").to_string();
            let suffix = row.read::<&str, _>("suffix").to_string();
            let platform_str = row.read::<&str, _>("platform").to_string();
            let platform = match Platform::from_str(platform_str.as_str()) {
                Ok(platform) => platform,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let submit_id = row.read::<&str, _>("submit_id").to_string();
            let submit_description = row.read::<&str, _>("submit_description").to_string();
            let identifier_str = row.read::<&str, _>("identifier").to_string();
            let identifier = match ProgramLanguage::from_str(identifier_str.as_str()) {
                Ok(identifier) => identifier,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            vec.push(LanguageSubmitConfig {
                alias,
                suffix,
                platform,
                identifier,
                submit_id,
                submit_description,
            })
        }
        return Ok(vec);
    }
    pub fn get_language_platform_config(
        &self,
        language: ProgramLanguage,
        platform: Platform,
    ) -> Result<Vec<LanguageConfig>, String> {
        let query = String::from("SELECT alias, suffix, platform, identifier, submit_id, submit_description, template_path, compile_command, execute_command, clear_command FROM language WHERE identifier = ? and platform = ?");
        let language_str = language.to_string();
        let platform_str = platform.to_string();
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        if let Err(info) = stmt.bind((1, language_str.as_str())) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((2, platform_str.as_str())) {
            return Err(info.to_string());
        }
        let mut res = Vec::new();
        for item in stmt.into_iter().filter_map(|x| match x {
            Ok(x) => Some(x),
            Err(_) => None,
        }) {
            let alias = item.read::<&str, _>("alias").to_string();
            let suffix = item.read::<&str, _>("suffix").to_string();
            let platform_str = item.read::<&str, _>("platform").to_string();
            let platform = match Platform::from_str(platform_str.as_str()) {
                Ok(platform) => platform,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let identifier_str = item.read::<&str, _>("identifier").to_string();
            let identifier = match ProgramLanguage::from_str(identifier_str.as_str()) {
                Ok(identifier) => identifier,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let submit_id = item.read::<&str, _>("submit_id").to_string();
            let submit_description = item.read::<&str, _>("submit_description").to_string();
            let template_path = item.read::<&str, _>("template_path").to_string();
            let compile_command = item.read::<&str, _>("compile_command").to_string();
            let execute_command = item.read::<&str, _>("execute_command").to_string();
            let clear_command = item.read::<&str, _>("clear_command").to_string();
            let language_config = LanguageConfig {
                alias,
                suffix,
                platform,
                identifier,
                submit_id,
                submit_description,
                template_path,
                compile_command,
                execute_command,
                clear_command,
            };
            res.push(language_config);
        }
        return Ok(res);
    }
    pub fn get_language_config(&self, language: ProgramLanguage) -> Result<LanguageConfig, String> {
        let query = String::from("SELECT alias, suffix, platform, identifier, submit_id, submit_description, template_path, compile_command, execute_command, clear_command FROM language WHERE identifier = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let language_str = language.to_string();
        if let Err(info) = stmt.bind((1, language_str.as_str())) {
            return Err(info.to_string());
        }
        let mut res = Vec::new();
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let alias = row.read::<&str, _>("alias").to_string();
            let suffix = row.read::<&str, _>("suffix").to_string();
            let platform_str = row.read::<&str, _>("platform").to_string();
            let platform = match Platform::from_str(platform_str.as_str()) {
                Ok(platform) => platform,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let identifier_str = row.read::<&str, _>("identifier").to_string();
            let identifier = match ProgramLanguage::from_str(identifier_str.as_str()) {
                Ok(identifier) => identifier,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let submit_id = row.read::<&str, _>("submit_id").to_string();
            let submit_description = row.read::<&str, _>("submit_description").to_string();
            let template_path = row.read::<&str, _>("template_path").to_string();
            let compile_command = row.read::<&str, _>("compile_command").to_string();
            let execute_command = row.read::<&str, _>("execute_command").to_string();
            let clear_command = row.read::<&str, _>("clear_command").to_string();
            res.push(LanguageConfig {
                alias,
                suffix,
                platform,
                identifier,
                submit_id,
                submit_description,
                template_path,
                compile_command,
                execute_command,
                clear_command,
            });
        }
        match res.len() {
            0 => {
                return Err(format!(
                    "Cannot find language config for {}",
                    language.to_string()
                ));
            }
            1 => {
                return Ok(res[0].clone());
            }
            _ => {
                let language_config = match Select::new("Please select language", res).prompt() {
                    Ok(language_config) => language_config,
                    Err(info) => {
                        return Err(info.to_string());
                    }
                };
                return Ok(language_config);
            }
        }
    }
    pub fn list_lang_config(&self) -> Result<(), String> {
        let query = String::from("SELECT alias, suffix, platform, identifier, submit_id, submit_description, template_path, compile_command, execute_command, clear_command FROM language");
        let stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
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
        for row in stmt.into_iter().filter_map(|x| match x {
            Ok(x) => Some(x),
            Err(_) => None,
        }){
            let alias = row.read::<&str, _>("alias").to_string();
            let suffix = row.read::<&str, _>("suffix").to_string();
            let platform_str = row.read::<&str, _>("platform").to_string();
            let platform = match Platform::from_str(platform_str.as_str()) {
                Ok(platform) => platform,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let identifier_str = row.read::<&str, _>("identifier").to_string();
            let identifier = match ProgramLanguage::from_str(identifier_str.as_str()) {
                Ok(identifier) => identifier,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let submit_id = row.read::<&str, _>("submit_id").to_string();
            let submit_description = row.read::<&str, _>("submit_description").to_string();
            let template_path = row.read::<&str, _>("template_path").to_string();
            let compile_command = row.read::<&str, _>("compile_command").to_string();
            let execute_command = row.read::<&str, _>("execute_command").to_string();
            let clear_command = row.read::<&str, _>("clear_command").to_string();
            table.add_row(row![
                alias,
                suffix,
                platform,
                identifier,
                submit_id,
                submit_description,
                template_path,
                compile_command,
                execute_command,
                clear_command
            ]);
        }
        table.printstd();
        return Ok(())
    }
    pub fn set_lang_config(
        &self,
        alias: &str,
        suffix: &str,
        platform: Platform,
        language_identifier: ProgramLanguage,
        submit_id: &str,
        submit_description: &str,
        template_path: &str,
        compile_command: &str,
        execute_command: &str,
        clear_command: &str,
    ) -> Result<(), String> {
        let query = String::from("INSERT OR REPLACE INTO language (alias, suffix, platform, identifier, submit_id, submit_description, template_path, compile_command, execute_command, clear_command) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        if let Err(info) = stmt.bind((1, alias)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((2, suffix)) {
            return Err(info.to_string());
        }
        let platform_str = platform.to_string();
        if let Err(info) = stmt.bind((3, platform_str.as_str())) {
            return Err(info.to_string());
        }
        let language_str = language_identifier.to_string();
        if let Err(info) = stmt.bind((4, language_str.as_str())) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((5, submit_id)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((6, submit_description)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((7, template_path)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((8, compile_command)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((9, execute_command)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((10, clear_command)) {
            return Err(info.to_string());
        }
        return match stmt.next() {
            Ok(_) => Ok(()),
            Err(info) => Err(info.to_string()),
        };
    }
}
