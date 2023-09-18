use lazy_static::lazy_static;
use sqlite;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;

lazy_static! {
    static ref CONFIG_DB: ConfigDatabase = ConfigDatabase::new();
}

pub struct ConfigDatabase {
    pub connection: sqlite::ConnectionWithFullMutex,
}

const INIT_QUERY: &str = "
CREATE TABLE IF NOT EXISTS config (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
CREATE TABLE IF NOT EXISTS account (id INTEGER PRIMARY KEY AUTOINCREMENT, platform TEXT, username TEXT, password TEXT, cookies TEXT default \"\", current INTEGER DEFAULT 0);
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
    /// Get the current account of the platform.
    #[allow(unused)]
    pub fn get_accounts(&self, platform: &str) -> Vec<[String; 4]> {
        let query =
            format!("SELECT username, password, cookies, current FROM account WHERE platform = ?");
        let mut res = Vec::new();
        for row in self
            .connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .bind((1, platform))
            .unwrap()
            .map(|row| row.unwrap())
        {
            let mut account = [String::new(), String::new(), String::new(), String::new()];
            account[0] = row.read::<&str, _>("username").to_string();
            account[1] = row.read::<&str, _>("password").to_string();
            account[2] = row.read::<&str, _>("cookies").to_string();
            account[3] = row.read::<i64, _>("current").to_string();
            res.push(account);
        }
        return res;
    }
    /// Add an account to the database.
    #[allow(unused)]
    pub fn add_account(
        &self,
        platform: &str,
        username: &str,
        password: &str,
    ) -> Result<String, String> {
        let query = format!("INSERT INTO account (platform, username, password) VALUES (?, ?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, platform)) {
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
        return Ok(String::new());
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
    let config_db_path = Path::new("test_config.sqlite");
    let config_db = ConfigDatabase::create_from_path(config_db_path);
    let _ = config_db.add_account("codeforces", "dianhsu", "xudian");
    let res = config_db.get_accounts("codeforces");
    for account in res {
        println!("{:?}", account);
    }
}
