mod contest;
use crate::{http, tool::Tool};

use colored::Colorize;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
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

fn gen_ftaa() -> String {
    return random_str::get_string(18, true, false, true, false);
}
fn gen_bfaa() -> String {
    return "f1b3f18c715565b589b7823cda7448ce".to_string();
}
impl Account {
    fn check_login(&mut self, host: &str, client: &mut http::Client) -> bool {
        let login_page = format!("{}/edu/courses", host);
        let resp = match client.get(&login_page) {
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        let re = Regex::new("handle = \"([\\s\\S]+?)\"").unwrap();
        for cap in re.captures_iter(resp.as_str()) {
            if cap.len() >= 2 {
                return true;
            }
        }
        return false;
    }
    #[allow(unused)]
    pub fn login(&mut self, host: &str) -> bool {
        let mut client = http::Client::new(&self.cookies, host).unwrap();
        if self.check_login(host, &mut client) {
            return true;
        }
        let login_page = format!("{}/enter", host);
        let resp = match client.get(&login_page) {
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        let csrf = {
            let re = Regex::new("csrf='(.+?)'").unwrap();
            let mut res = String::new();
            for cap in re.captures_iter(resp.as_str()) {
                res = cap[1].to_string();
            }
            res
        };
        log::debug!("csrf: {}", csrf);
        self.bfaa = gen_bfaa();
        self.ftaa = gen_ftaa();
        let res = match client.post_form(
            &login_page,
            &[
                ("csrf_token", &csrf),
                ("action", "enter"),
                ("ftaa", &self.ftaa),
                ("bfaa", &self.bfaa),
                ("handleOrEmail", &self.handle),
                ("password", &self.password),
                ("_tta", "176"),
                ("remember", "on"),
            ],
        ) {
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        if (self.check_login(host, &mut client)) {
            self.cookies = client.save_cookies();
            return true;
        } else {
            println!("Login failed");
            return false;
        }
    }
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
    pub fn check_accounts(&mut self){
        for idx in 0..self.accounts.len(){
            let mut client = http::Client::new(&self.accounts[idx].cookies, &self.host).unwrap();
            let res = self.accounts[idx].check_login(&self.host, &mut client);
            println!("{}) try login account {}, result: {}", idx, self.accounts[idx].handle, res);
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
