use lazy_static::lazy_static;
use sqlite::{self};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;
mod account;
mod config;
mod language;
use crate::logger::LOGGER as logger;
lazy_static! {
    pub static ref CONFIG_DB: ConfigDatabase = ConfigDatabase::new();
}

pub struct ConfigDatabase {
    pub connection: sqlite::ConnectionWithFullMutex,
}

const INIT_QUERY: &str = "
CREATE TABLE IF NOT EXISTS config (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
CREATE TABLE IF NOT EXISTS account (id INTEGER PRIMARY KEY AUTOINCREMENT, platform TEXT, username TEXT, password TEXT, cookies TEXT default \"\", last_use TEXT default \"1970-01-01T00:00:00+00:00\", current INTEGER DEFAULT 0);
CREATE TABLE IF NOT EXISTS language (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
";
impl ConfigDatabase {
    pub fn create_from_path(config_path: &Path) -> Self {
        let connection: sqlite::ConnectionWithFullMutex =
            match sqlite::Connection::open_with_full_mutex(config_path) {
                Ok(conn) => conn,
                Err(info) => {
                    logger.error(info.to_string().as_str());
                    exit(1);
                }
            };
        match connection.execute(INIT_QUERY) {
            Ok(_) => {}
            Err(info) => {
                logger.error(info.to_string().as_str());
                exit(1);
            }
        }
        Self { connection }
    }
    pub fn new() -> Self {
        let pathbuf = match home::home_dir() {
            Some(pathbuf) => pathbuf,
            None => {
                logger.error("Cannot get home directory");
                exit(1);
            }
        };
        let config_dir = pathbuf.join(".ace");
        match create_dir_all(&config_dir) {
            Ok(_) => {}
            Err(info) => {
                logger.error(info.to_string().as_str());
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
}
