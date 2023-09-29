use lazy_static::lazy_static;
use sqlite::{self};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;
mod account;

lazy_static! {
    pub static ref CONFIG_DB: ConfigDatabase = ConfigDatabase::new();
}

pub struct ConfigDatabase {
    pub connection: sqlite::ConnectionWithFullMutex,
}

const INIT_QUERY: &str = "
CREATE TABLE IF NOT EXISTS config (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
CREATE TABLE IF NOT EXISTS account (id INTEGER PRIMARY KEY AUTOINCREMENT, platform TEXT, username TEXT, password TEXT, cookies TEXT default \"\", last_use TEXT default \"1970-01-01T00:00:00+00:00\", current INTEGER DEFAULT 0);
";
impl ConfigDatabase {
    pub fn create_from_path(config_path: &Path) -> Self {
        let connection: sqlite::ConnectionWithFullMutex =
            match sqlite::Connection::open_with_full_mutex(config_path) {
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
        Self { connection }
    }
    pub fn new() -> Self {
        let pathbuf = home::home_dir().unwrap();
        let config_dir = pathbuf.join(".ace");
        create_dir_all(&config_dir).unwrap();

        let binding = config_dir.join("config.sqlite");
        let config_path = binding.as_path();
        return Self::create_from_path(config_path);
    }

    #[allow(unused)]
    pub fn get_config(&self, name: &str) -> Vec<String> {
        let query = format!("SELECT value FROM config WHERE name = ?");
        let mut res = Vec::new();
        for row in self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .bind((1, name))
            .unwrap()
            .map(|row| row.unwrap())
        {
            res.push(row.read::<&str, _>("value").to_string());
        }
        return res;
    }
    #[allow(unused)]
    pub fn set_config(&self, name: &str, value: &str) -> bool {
        let query = format!("INSERT OR REPLACE INTO config (name, value) VALUES (?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(_) => return false,
        };
        match stmt.bind((1, name)) {
            Ok(_) => {}
            Err(_) => return false,
        };
        match stmt.bind((2, value)) {
            Ok(_) => {}
            Err(_) => return false,
        };
        match stmt.next() {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }
}

#[test]
fn test_config_database() {
    let path_str = format!("test_{}.sqlite", rand::random::<u64>());
    let config_db_path = Path::new(&path_str);
    let config_db = ConfigDatabase::create_from_path(config_db_path);
    let mut res = config_db.add_account(crate::config::Platform::Codeforces, "dianhsu", "xudian");
    assert_eq!(res.is_ok(), true);
    res = config_db.add_account(crate::config::Platform::Codeforces, "dianhsu", "xudian");
    assert_eq!(res.is_ok(), false);
    let res = config_db.get_accounts(Some(crate::config::Platform::Codeforces));
    for account in res {
        println!("{:?}", account);
    }
}
