use crate::{http, tool::Tool};

use regex::Regex;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Account {
    pub handle: String,
    pub password: String,
    pub ftaa: String,
    pub bfaa: String,
    pub cookies: String,
}

#[derive(Serialize, Deserialize)]
pub struct Codeforces {
    pub host: String,
    pub accounts: Vec<Account>,
}

impl Account {
    #[allow(unused)]
    pub fn login(&mut self, host: &str) -> bool {
        let mut client = http::Client::new(&self.cookies, host).unwrap();
        let login_page = format!("{}/enter", host);
        let resp = match client.get(&login_page) {
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        if resp.find("Redirecting... Please, wait.").is_some() {
            let re = Regex::new("var a=toNumbers\\(\"([0-9a-f]*)\"\\),b=toNumbers\\(\"([0-9a-f]*)\"\\),c=toNumbers\\(\"([0-9a-f]*)\"\\);").unwrap();
            let mut a = String::new();
            let mut b = String::new();
            let mut c = String::new();
            for cap in re.captures_iter(resp.as_str()) {
                a = cap[1].to_string();
                b = cap[2].to_string();
                c = cap[3].to_string();
            }
        }

        self.cookies = client.save_cookies();

        return true;
    }
}
impl Codeforces {
    pub fn new() -> Codeforces {
        Codeforces {
            host: "https://codeforces.com".to_string(),
            accounts: Vec::new(),
        }
    }
    pub fn list_accont(&mut self) {
        for idx in 0..self.accounts.len() {
            println!("{}: {}", idx, self.accounts[idx].handle)
        }
    }
    pub fn add_account(&mut self) -> Result<Account, String> {
        let handle = Tool::get_input("Please input your handle name or email address: ");
        let password = Tool::get_password_input("Please input your password: ");
        let mut account = Account {
            handle,
            password,
            ftaa: String::new(),
            bfaa: String::new(),
            cookies: String::new(),
        };

        if account.login(&self.host) {
            Ok(account)
        } else {
            Err("login failed".to_string())
        }
    }
    #[allow(unused)]
    pub fn parse(&mut self) {
        unimplemented!()
    }
    #[allow(unused)]
    pub fn submit(&mut self) {}
}
