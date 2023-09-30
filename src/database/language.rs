use prettytable::{Cell, Row, Table};

use super::ConfigDatabase;
use crate::constants::ProgramLanguage;
use crate::model::LanguageConfig;

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
