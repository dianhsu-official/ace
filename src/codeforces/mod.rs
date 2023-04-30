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

fn gen_ftaa() -> String {
    return random_str::get_string(18, true, false, true, false);
}
fn gen_bfaa() -> String {
    return "f1b3f18c715565b589b7823cda7448ce".to_string();
}
impl Account {
    fn check_login(&mut self, host: &str) -> bool{
        let mut client = http::Client::new(&self.cookies, host).unwrap();
        let login_page = format!("{}/edu/courses", host);
        let resp = match client.get(&login_page) {
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        let re = Regex::new("handle = \"([\\s\\S]+?)\"").unwrap();
        for cap in re.captures_iter(resp.as_str()) {
            if cap.len() >= 2{
                return true;
            }
        }
        return false;
    }
    #[allow(unused)]
    pub fn login(&mut self, host: &str) -> bool {
        if self.check_login(host) {
            return true;
        }
        let mut client = http::Client::new(&self.cookies, host).unwrap();
        let login_page = format!("{}/enter", host);
        let resp = match client.get(&login_page) {
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        // if resp.find("Redirecting... Please, wait.").is_some() {
        //     let re = Regex::new("var a=toNumbers\\(\"([0-9a-f]*)\"\\),b=toNumbers\\(\"([0-9a-f]*)\"\\),c=toNumbers\\(\"([0-9a-f]*)\"\\);").unwrap();
        //     let mut a = String::new();
        //     let mut b = String::new();
        //     let mut c = String::new();
        //     for cap in re.captures_iter(resp.as_str()) {
        //         a = cap[1].to_string();
        //         b = cap[2].to_string();
        //         c = cap[3].to_string();
        //     }
        // }
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
        ){
            Ok(resp) => resp,
            Err(_) => {
                return false;
            }
        };
        if(self.check_login(host)){
            self.cookies = client.save_cookies();
            return true;
        }else{
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
