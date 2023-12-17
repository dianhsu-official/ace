use lazy_static::lazy_static;
use sqlite::{self};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;
mod account;
mod config;
mod language;
lazy_static! {
    pub static ref CONFIG_DB: ConfigDatabase = ConfigDatabase::new();
}

pub struct ConfigDatabase {
    pub connection: sqlite::ConnectionThreadSafe,
}

const INIT_QUERY: &str = "
CREATE TABLE IF NOT EXISTS config (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
CREATE TABLE IF NOT EXISTS account (id INTEGER PRIMARY KEY AUTOINCREMENT, platform TEXT, username TEXT, password TEXT, cookies TEXT default \"\", last_use TEXT default \"1970-01-01T00:00:00+00:00\", current INTEGER DEFAULT 0);
CREATE TABLE IF NOT EXISTS language (
    id INTEGER PRIMARY KEY AUTOINCREMENT, 
    alias TEXT, 
    suffix TEXT, 
    platform TEXT, 
    identifier TEXT, 
    submit_id TEXT, 
    submit_description TEXT, 
    template_path TEXT default \"\", 
    compile_command TEXT default \"\", 
    execute_command TEXT default \"\", 
    clear_command TEXT default \"\"
);
";
impl ConfigDatabase {
    pub fn create_from_path(config_path: &Path) -> Self {
        let connection: sqlite::ConnectionThreadSafe =
            match sqlite::Connection::open_thread_safe(config_path) {
                Ok(conn) => conn,
                Err(info) => {
                    log::error!("{}", info);
                    exit(1);
                }
            };
        match connection.execute(INIT_QUERY) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                exit(1);
            }
        }
        let workspace = match home::home_dir() {
            Some(home_path) => {
                let workspace = home_path.join("ace");
                match workspace.to_str() {
                    Some(workspace) => Some(workspace.to_string()),
                    None => None,
                }
            }
            None => None,
        };
        if let Some(workspace_dir) = workspace {
            let query = format!(
                "INSERT OR IGNORE INTO config (name, value) VALUES ('workspace', '{}')",
                workspace_dir
            );
            match connection.execute(query.as_str()) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        Self { connection }
    }
    pub fn new() -> Self {
        let pathbuf = match home::home_dir() {
            Some(pathbuf) => pathbuf,
            None => {
                exit(1);
            }
        };
        let config_dir = pathbuf.join(".ace");
        match create_dir_all(&config_dir) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                exit(1);
            }
        }

        let binding = config_dir.join("config.sqlite");
        let config_path = binding.as_path();
        return Self::create_from_path(config_path);
    }
}

#[test]
fn test_config_database() {
    let path_str = format!("test_{}.sqlite", rand::random::<u64>());
    let config_db_path = Path::new(&path_str);
    let config_db = ConfigDatabase::create_from_path(config_db_path);
    let mut res = config_db.add_account(crate::model::Platform::Codeforces, "dianhsu", "xudian");
    assert_eq!(res.is_ok(), true);
    res = config_db.add_account(crate::model::Platform::Codeforces, "dianhsu", "xudian");
    assert_eq!(res.is_ok(), false);
    let res = config_db.get_accounts(Some(crate::model::Platform::Codeforces));
    for account in res {
        println!("{:?}", account);
    }
    drop(config_db);
    std::fs::remove_file(config_db_path).unwrap();
}
