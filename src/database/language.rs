use prettytable::{Cell, Row, Table};
use serde_derive::{Deserialize, Serialize};

use super::ConfigDatabase;
use crate::constants::ProgramLanguage;
use crate::model::{LanguageConfig, Platform};
use std::str::FromStr;
impl ConfigDatabase {
    pub fn get_program_language_from_suffix(
        &mut self,
        suffix: &str,
    ) -> Result<ProgramLanguage, String> {
        let query = format!("SELECT key FROM language WHERE suffix = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, suffix)) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let name = row.read::<&str, _>("key").to_string();
            return Ok(ProgramLanguage::from_str(name.as_str()).unwrap());
        }
        return Err(format!("Please set language config for {} first.", suffix));
    }
    pub fn get_language_platform_config(
        &self,
        language: ProgramLanguage,
        platform: Platform,
    ) -> Result<Vec<LanguageConfig>, String> {
        let query = format!("SELECT alias,key, submit_language_id, submit_language_description, platform, suffix, template_path, compile_command, execute_command, clear_command FROM language WHERE language = ? AND platform = ?");
        let language_str = language.to_string();
        let platform_str = platform.to_string();
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, language_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let mut res = Vec::new();
        for item in stmt.into_iter().filter_map(|x| match x {
            Ok(x) => Some(x),
            Err(_) => None,
        }) {
            let alias = item.read::<&str, _>("alias").to_string();
            let key = item.read::<&str, _>("key").to_string();
            let submit_language_id = item.read::<&str, _>("submit_language_id").to_string();
            let submit_language_description = item
                .read::<&str, _>("submit_language_description")
                .to_string();
            let platform_str = item.read::<&str, _>("platform").to_string();
            let platform = match Platform::from_str(platform_str.as_str()) {
                Ok(platform) => platform,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let suffix = item.read::<&str, _>("suffix").to_string();
            let template_path = item.read::<&str, _>("template_path").to_string();
            let compile_command = item.read::<&str, _>("compile_command").to_string();
            let execute_command = item.read::<&str, _>("execute_command").to_string();
            let clear_command = item.read::<&str, _>("clear_command").to_string();
            let language_config = LanguageConfig {
                alias,
                key,
                suffix,
                platform,
                submit_language_id,
                submit_language_description,
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
        let query = format!("SELECT alias,key, submit_language_id, submit_language_description, platform, suffix, template_path, compile_command, execute_command, clear_command FROM language WHERE name = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let language_str = language.to_string();
        match stmt.bind((1, language_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let alias = row.read::<&str, _>("alias").to_string();
            let key = row.read::<&str, _>("key").to_string();
            let submit_language_id = row.read::<&str, _>("submit_language_id").to_string();
            let submit_language_description = row
                .read::<&str, _>("submit_language_description")
                .to_string();
            let platform_str = row.read::<&str, _>("platform").to_string();
            let platform = match Platform::from_str(platform_str.as_str()) {
                Ok(platform) => platform,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            let suffix = row.read::<&str, _>("suffix").to_string();
            let template_path = row.read::<&str, _>("template_path").to_string();
            let compile_command = row.read::<&str, _>("compile_command").to_string();
            let execute_command = row.read::<&str, _>("execute_command").to_string();
            let clear_command = row.read::<&str, _>("clear_command").to_string();
            return Ok(LanguageConfig {
                alias,
                key,
                suffix,
                platform,
                submit_language_id,
                submit_language_description,
                template_path,
                compile_command,
                execute_command,
                clear_command,
            });
        }
        return Err(format!(
            "Please set language config for {} first.",
            language
        ));
    }
    pub fn list_lang_config(&self) -> Result<(), String> {
        let query = format!("SELECT alias, key, platform, suffix, template_path, compile_command, execute_command, clear_command FROM language");
        let stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let mut table: Table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("alias"),
            Cell::new("key"),
            Cell::new("platform"),
            Cell::new("suffix"),
            Cell::new("template_path"),
            Cell::new("compile_command"),
            Cell::new("execute_command"),
            Cell::new("clear_command"),
        ]));
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let alias = row.read::<&str, _>("alias").to_string();
            let key = row.read::<&str, _>("key").to_string();
            let platform_str = row.read::<&str, _>("platform").to_string();
            let suffix = row.read::<&str, _>("suffix").to_string();
            let template_path = row.read::<&str, _>("template_path").to_string();
            let compile_command = row.read::<&str, _>("compile_command").to_string();
            let execute_command = row.read::<&str, _>("execute_command").to_string();
            let clear_command = row.read::<&str, _>("clear_command").to_string();

            table.add_row(Row::new(vec![
                Cell::new(alias.as_str()),
                Cell::new(key.as_str()),
                Cell::new(platform_str.as_str()),
                Cell::new(suffix.as_str()),
                Cell::new(template_path.as_str()),
                Cell::new(compile_command.as_str()),
                Cell::new(execute_command.as_str()),
                Cell::new(clear_command.as_str()),
            ]));
        }
        table.printstd();
        return Ok(());
    }
    pub fn set_lang_config(
        &self,
        lang: ProgramLanguage,
        suffix: &str,
        template_path: &str,
        compile_command: &str,
        execute_command: &str,
        clear_command: &str,
    ) -> Result<(), String> {
        let query = format!("INSERT OR REPLACE INTO language (name, suffix, template_path, compile_command, execute_command, clear_command) VALUES (?, ?, ?, ?, ?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => return Err(info.to_string()),
        };
        let name = lang.to_string();
        match stmt.bind((1, name.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        if let Err(info) = stmt.bind((2, suffix)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((3, template_path)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((4, compile_command)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((5, execute_command)) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((6, clear_command)) {
            return Err(info.to_string());
        }
        match stmt.next() {
            Ok(_) => {
                return Ok(());
            }
            Err(info) => {
                return Err(info.to_string());
            }
        };
    }
}
