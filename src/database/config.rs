use super::ConfigDatabase;

impl ConfigDatabase {
    pub fn create_config(&self, name: &str, value: &str) -> Result<(), String> {
        if self.get_config(name).is_ok() {
            return Err(String::from("Config already exists"));
        }
        let query = format!("INSERT INTO config (name, value) VALUES (?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, name)) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, value)) {
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
    pub fn get_config(&self, name: &str) -> Result<String, String> {
        let query = format!("SELECT value FROM config WHERE name = ?");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((1, name)) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        for row in stmt.into_iter().filter_map(|row| match row {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            return Ok(row.read::<&str, _>("value").to_string());
        }
        return Err(String::from("No such config"));
    }
    pub fn list_config(&self) -> Vec<[String; 2]> {
        let query = format!("SELECT name, value FROM config");
        let mut res = Vec::new();
        let stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(_) => return res,
        };
        for row in stmt.into_iter().filter_map(|x| match x {
            Ok(row) => Some(row),
            Err(_) => None,
        }) {
            let mut item = [String::new(), String::new()];
            item[0] = row.read::<&str, _>("name").to_string();
            item[1] = row.read::<&str, _>("value").to_string();
            res.push(item);
        }
        return res;
    }
    pub fn remove_config(&self, names: Vec<String>) -> Result<(), String> {
        let query = format!("DELETE FROM config WHERE name in ('{}')", names.join("','"));
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
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
    pub fn set_config(&self, name: &str, value: &str) -> Result<(), String> {
        let query = format!("INSERT OR REPLACE INTO config (name, value) VALUES (?, ?)");
        let mut stmt = match self.connection.prepare(query) {
            Ok(stmt) => stmt,
            Err(info) => return Err(info.to_string()),
        };
        match stmt.bind((1, name)) {
            Ok(_) => {}
            Err(info) => {
                return Err(info.to_string());
            }
        };
        match stmt.bind((2, value)) {
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
