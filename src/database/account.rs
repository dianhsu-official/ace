use crate::model::{AccountInfo, Platform};

use super::ConfigDatabase;
impl ConfigDatabase {
    pub fn remove_accounts(&self, platform: Platform, usernames: Vec<String>) {
        let query = format!(
            "DELETE FROM account WHERE platform = ? and username in ('{}')",
            usernames.join("','")
        );
        let mut stmt = match self.connection.prepare(query) {
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
    pub fn get_default_account(&self, platform: Platform) -> Result<AccountInfo, String> {
        let platform_str = platform.to_string();
        let mut stmt = match self.connection.prepare("SELECT username, password, cookies, current, last_use FROM account WHERE platform = ? AND current = 1") {
            Ok(stmt) => stmt,
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let mut account = AccountInfo {
                username: String::new(),
                password: String::new(),
                cookies: String::new(),
                current: 0,
                last_use: String::new(),
            };
            account.username = row.read::<&str, _>("username").to_string();
            account.password = row.read::<&str, _>("password").to_string();
            account.cookies = row.read::<&str, _>("cookies").to_string();
            account.current = row.read::<i64, _>("current");
            account.last_use = row.read::<&str, _>("last_use").to_string();
            return Ok(account);
        }
        return Err("No default account found.".to_string());
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

    pub fn set_default_account(&self, platform: Platform, username: &str) -> Result<(), String> {
        let clear_query = format!("UPDATE account SET current = 0 WHERE platform = ?");
        let mut stmt = match self.connection.prepare(clear_query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        let platform_str = platform.to_string();
        match stmt.bind((1, platform_str.as_str())) {
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
        let set_query =
            format!("UPDATE account SET current = 1 WHERE platform = ? AND username = ?");
        let mut stmt = match self.connection.prepare(set_query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, platform_str.as_str())) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, username)) {
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

    pub fn update_password(
        &self,
        platform: &str,
        username: &str,
        password: &str,
    ) -> Result<(), String> {
        let query = format!("UPDATE account SET password = ? WHERE platform = ? AND username = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, password)) {
            Ok(_) => {}
            Err(info) => {
                log::error!("{}", info);
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, platform)) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((3, username)) {
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
