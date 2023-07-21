mod account;
mod contest;
use crate::misc::http;
use crate::misc::tool::Tool;

use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};

use self::account::Account;

#[derive(Serialize, Deserialize)]
pub struct Codeforces {
    pub host: String,
    pub accounts: Vec<Account>,
}
impl Codeforces {
    pub fn new() -> Codeforces {
        Codeforces {
            host: "https://codeforces.com".to_string(),
            accounts: Vec::new(),
        }
    }
    pub fn list_accounts(&mut self) {
        for idx in 0..self.accounts.len() {
            println!("{}: {}", idx, self.accounts[idx].handle)
        }
    }
    pub fn check_accounts(&mut self) {
        for idx in 0..self.accounts.len() {
            let mut client = http::Client::new(&self.accounts[idx].cookies, &self.host).unwrap();
            let res = self.accounts[idx].check_login(&self.host, &mut client);
            println!(
                "{}) try login account {}, result: {}",
                idx, self.accounts[idx].handle, res
            );
        }
    }
    pub fn add_account(&mut self) -> Result<Account, String> {
        let handle = Tool::get_input("Please input your handle name or email address");
        let password = Tool::get_password_input("Please input your password");
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
    pub fn parse(&mut self, identifier: &str) {
        let url = format!("{}/contest/{}", self.host, identifier);
        println!("Parse contest from {}", url.green());
    }
    #[allow(unused)]
    pub fn submit(&mut self) {
        unimplemented!()
    }
}
