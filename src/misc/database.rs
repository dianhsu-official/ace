use lazy_static::lazy_static;
use sqlite;
use std::fs::create_dir_all;
use std::process::exit;

lazy_static! {
    static ref CONFIG_DB: ConfigDatabase = ConfigDatabase::new();
}

pub struct ConfigDatabase {
    pub connection: sqlite::ConnectionWithFullMutex,
}

const INIT_QUERY: &str = "
CREATE TABLE IF NOT EXISTS config (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
CREATE TABLE IF NOT EXISTS account (id INTEGER PRIMARY KEY AUTOINCREMENT, platform TEXT, username TEXT, password TEXT, cookies TEXT, current INTEGER DEFAULT 0);
";
impl ConfigDatabase {
    pub fn new() -> Self {
        let pathbuf = home::home_dir().unwrap();
        let config_dir = pathbuf.join(".ace");
        create_dir_all(&config_dir).unwrap();

        let binding = config_dir.join("config.db");
        let config_path = binding.as_path();
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
