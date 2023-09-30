use prettytable::{Cell, Row, Table};
use serde_derive::{Deserialize, Serialize};

use super::ConfigDatabase;
use crate::constants::ProgramLanguage;
use crate::model::{LanguageConfig, Platform};
use std::str::FromStr;
impl ConfigDatabase {
    pub fn get_language_config(&self, language: ProgramLanguage) -> Result<LanguageConfig, String> {
        let query = format!("SELECT suffix, template_path, compile_command, execute_command, clear_command FROM language WHERE name = ?");
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
            let suffix = row.read::<&str, _>("suffix").to_string();
            let template_path = row.read::<&str, _>("template_path").to_string();
            let compile_command = row.read::<&str, _>("compile_command").to_string();
            let execute_command = row.read::<&str, _>("execute_command").to_string();
            let clear_command = row.read::<&str, _>("clear_command").to_string();
            return Ok(LanguageConfig {
                suffix,
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
        let query = format!("SELECT name, suffix, template_path, compile_command, execute_command, clear_command FROM language");
        let stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let mut table: Table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("name"),
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
            let name = row.read::<&str, _>("name").to_string();
            let suffix = row.read::<&str, _>("suffix").to_string();
            let template_path = row.read::<&str, _>("template_path").to_string();
            let compile_command = row.read::<&str, _>("compile_command").to_string();
            let execute_command = row.read::<&str, _>("execute_command").to_string();
            let clear_command = row.read::<&str, _>("clear_command").to_string();

            table.add_row(Row::new(vec![
                Cell::new(name.as_str()),
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

impl ConfigDatabase {
    pub fn get_program_language_from_suffix(
        &self,
        suffix: &str,
    ) -> Result<ProgramLanguage, String> {
        let query = format!("SELECT name FROM language WHERE suffix = ?");
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
            let name = row.read::<&str, _>("name").to_string();
            match ProgramLanguage::from_str(name.as_str()) {
                Ok(program_language) => {
                    return Ok(program_language);
                }
                Err(info) => {
                    return Err(info.to_string());
                }
            }
        }
        return Err(format!(
            "Please set language config for suffix {} first.",
            suffix
        ));
    }
    #[allow(dead_code)]
    pub fn set_language_platform_submit_lang_id(
        &mut self,
        language: ProgramLanguage,
        platform: Platform,
        lang_id: &str,
    ) -> Result<LanguageExt, String> {
        let mut language_ext = match self.get_language_platform_submit_lang_id(language) {
            Ok(language_ext) => language_ext,
            Err(_) => LanguageExt {
                codeforces: None,
                atcoder: None,
            },
        };
        match platform {
            Platform::Codeforces => {
                language_ext.codeforces = Some(lang_id.to_string());
            }
            Platform::AtCoder => {
                language_ext.atcoder = Some(lang_id.to_string());
            }
        }
        let language_ext_str = match serde_json::to_string(&language_ext) {
            Ok(language_ext_str) => language_ext_str,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let query = String::from("INSERT OR REPLACE INTO language_ext (name, value) VALUES (?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        if let Err(info) = stmt.bind((1, language.to_string().as_str())) {
            return Err(info.to_string());
        }
        if let Err(info) = stmt.bind((2, language_ext_str.as_str())) {
            return Err(info.to_string());
        }
        match stmt.next() {
            Ok(_) => {
                return Ok(language_ext);
            }
            Err(info) => {
                return Err(info.to_string());
            }
        };
    }
    pub fn get_language_platform_submit_lang_id(
        &self,
        language: ProgramLanguage,
    ) -> Result<LanguageExt, String> {
        let query = String::from("SELECT value FROM language_ext WHERE name = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        if let Err(info) = stmt.bind((1, language.to_string().as_str())) {
            return Err(info.to_string());
        }
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let value = row.read::<&str, _>("value").to_string();
            let language_ext: LanguageExt = match serde_json::from_str(&value) {
                Ok(language_ext) => language_ext,
                Err(info) => {
                    return Err(info.to_string());
                }
            };
            return Ok(language_ext);
        }
        return Err(format!(
            "Please set language config for {} first.",
            language
        ));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageExt {
    pub codeforces: Option<String>,
    pub atcoder: Option<String>,
}
