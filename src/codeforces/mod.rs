use serde_derive::{Deserialize, Serialize};

use crate::platform::Platform;
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub handle: String,
    pub password: String,
    pub ftaa: String,
    pub bfaa: String,
}
#[derive(Serialize, Deserialize)]
pub struct Codeforces {
    pub host: String,
    pub accounts: Vec<Account>,
}
impl Account {
    pub fn new() -> Account {
        Account {
            handle: String::new(),
            password: String::new(),
            ftaa: String::new(),
            bfaa: String::new(),
        }
    }
    pub fn update(&mut self) {
        unimplemented!()
    }
}
impl Codeforces {
    pub fn new() -> Codeforces {
        Codeforces {
            host: "https://codeforces.com".to_string(),
            accounts: Vec::new(),
        }
    }
    pub fn login(&mut self) {
        unimplemented!()
    }
    pub fn parse(&mut self) {
        unimplemented!()
    }
    pub fn submit(&mut self) {}
}
