use crate::misc::http;
use crate::CONN;
use regex::Regex;
use sqlite::State;
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub handle: String,
    pub password: String,
    pub ftaa: String,
    pub bfaa: String,
    pub cookies: String,
}

fn gen_ftaa() -> String {
    return random_str::get_string(18, true, false, true, false);
}
fn gen_bfaa() -> String {
    return "f1b3f18c715565b589b7823cda7448ce".to_string();
}
impl Account {
    pub fn create() {
        let mut statement = CONN.prepare(
            "INSERT INTO account(platform, username, password, cookies) VALUES ('', '', '', '')").unwrap();
        statement.bind((1, "")).unwrap();
        statement.bind((2, "")).unwrap();
        statement.bind((3, "")).unwrap();
        statement.bind((4, "")).unwrap();
        statement.
    }
    pub fn check_login(&mut self, host: &str, client: &mut http::Client) -> bool {
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
