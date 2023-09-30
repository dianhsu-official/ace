use prettytable::{Cell, Row, Table};

use super::ConfigDatabase;
use crate::constants::ProgramLanguage;
use crate::model::LanguageConfig;

impl ConfigDatabase {
    pub fn list_lang_config(&self) -> Result<(), String> {
        let query = format!("SELECT name, value FROM language");
        let stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let mut table: Table = Table::new();
        table.add_row(Row::new(vec![Cell::new("Name"), Cell::new("Value")]));
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let name = row.read::<&str, _>("name").to_string();
            let raw_value = row.read::<&str, _>("value").to_string();
            let lang = match serde_json::from_str::<LanguageConfig>(raw_value.as_str()) {
                Ok(lang) => lang,
                Err(_) => continue,
            };
            let value = format!(
                "suffix: {}\ntemplate_path: {}\ncompile_command: {}\nexecute_command: {}\nclear_command: {}",
                lang.suffix, lang.template_path, lang.compile_command, lang.execute_command, lang.clear_command);
            table.add_row(Row::new(vec![
                Cell::new(name.as_str()),
                Cell::new(value.as_str()),
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
        let lang_value = LanguageConfig {
            suffix: suffix.to_string(),
            template_path: template_path.to_string(),
            compile_command: compile_command.to_string(),
            execute_command: execute_command.to_string(),
            clear_command: clear_command.to_string(),
        };
        let value = match serde_json::to_string(&lang_value) {
            Ok(value) => value,
            Err(_) => {
                return Err("Serialize error".to_string());
            }
        };
        let query = format!("INSERT OR REPLACE INTO language (name, value) VALUES (?, ?)");
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
        match stmt.bind((2, value.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.next() {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        return Ok(());
    }
}
