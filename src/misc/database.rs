use lazy_static::lazy_static;
use sqlite::{self};
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;

use crate::config::Platform;

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
    pub fn remove_accounts(&self, platform: Platform, usernames: Vec<String>) {
        let query = format!(
            "DELETE FROM account WHERE platform = ? and username in ('{}')",
            usernames.join("','")
        );
        let mut stmt = match CONFIG_DB.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                log::error!("{}", info);
                return;
            }
        };
        let platform_str = platform.to_string();
        match stmt.bind((1, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return;
            }
        };
        match stmt.next() {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return;
            }
        };
    }
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
    /// Get the current account of the platform.
    pub fn get_accounts(&self, platform: Option<Platform>) -> Vec<[String; 6]> {
        if let Some(platform_item) = platform {
            let query = format!(
            "SELECT username, password, cookies, current, last_use, platform FROM account WHERE platform = ?"
        );
            let platform_str = platform_item.to_string();
            let mut res = Vec::new();
            for row in self
                .connection
                .prepare(query)
                .unwrap()
                .into_iter()
                .bind((1, platform_str.as_str()))
                .unwrap()
                .map(|row| row.unwrap())
            {
                let mut account = [
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ];
                account[0] = row.read::<&str, _>("username").to_string();
                account[1] = row.read::<&str, _>("password").to_string();
                account[2] = row.read::<&str, _>("cookies").to_string();
                account[3] = row.read::<i64, _>("current").to_string();
                account[4] = row.read::<&str, _>("last_use").to_string();
                account[5] = row.read::<&str, _>("platform").to_string();
                res.push(account);
            }
            return res;
        } else {
            let query = format!(
                "SELECT username, password, cookies, current, last_use, platform FROM account"
            );
            let mut res = Vec::new();
            for row in self
                .connection
                .prepare(query)
                .unwrap()
                .into_iter()
                .map(|row| row.unwrap())
            {
                let mut account = [
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ];
                account[0] = row.read::<&str, _>("username").to_string();
                account[1] = row.read::<&str, _>("password").to_string();
                account[2] = row.read::<&str, _>("cookies").to_string();
                account[3] = row.read::<i64, _>("current").to_string();
                account[4] = row.read::<&str, _>("last_use").to_string();
                account[5] = row.read::<&str, _>("platform").to_string();
                res.push(account);
            }
            return res;
        }
    }

    /// Check if the account exists.
    pub fn is_account_exist(&self, platform: Platform, username: &str) -> Result<bool, String> {
        let query = format!("SELECT COUNT(*) FROM account WHERE platform = ? AND username = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        let platform_str = platform.to_string();
        match stmt.bind((1, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, username)) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.next() {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        let cnt = match stmt.read::<i64, _>(0) {
            Ok(cnt) => cnt,
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        if cnt > 0 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn save_cookies(
        &self,
        platform: Platform,
        username: &str,
        cookies: &str,
    ) -> Result<(), String> {
        let query = format!(
            "UPDATE account SET cookies = ?, last_use = ? WHERE platform = ? AND username = ?"
        );
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let platform_str = platform.to_string();
        match stmt.bind((1, cookies)) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        let now = chrono::Utc::now().to_rfc3339();
        match stmt.bind((2, now.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((3, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((4, username)) {
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
    /// Add an account to the database.
    pub fn add_account(
        &self,
        platform: Platform,
        username: &str,
        password: &str,
    ) -> Result<(), String> {
        match self.is_account_exist(platform, username) {
            Ok(true) => {
                return Err(format!("Account {} already exists.", username));
            }
            Ok(false) => {}
            Err(info) => {
                return Err(info);
            }
        };
        let query = format!("INSERT INTO account (platform, username, password) VALUES (?, ?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        let platform_str = platform.to_string();
        match stmt.bind((1, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, username)) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.bind((3, password)) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.next() {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        return Ok(());
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
    let mut res = config_db.add_account(Platform::Codeforces, "dianhsu", "xudian");
    assert_eq!(res.is_ok(), true);
    res = config_db.add_account(Platform::Codeforces, "dianhsu", "xudian");
    assert_eq!(res.is_ok(), false);
    let res = config_db.get_accounts(Some(Platform::Codeforces));
    for account in res {
        println!("{:?}", account);
    }
}
