use std::collections::HashMap;
mod config;
use crate::misc::http_client::HttpClient;
use crate::platform::lib::OnlineJudge;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use regex::Regex;
use soup::Soup;
pub struct Codeforces {
    pub client: HttpClient,
}

impl OnlineJudge for Codeforces {
    /// Submit code to the platform.  
    ///
    /// identifier: the identifier of the problem.  
    ///             For example, the identifier of the problem https://codeforces.com/problemset/problem/4/A is 4_A.  
    ///
    /// code: the code to submit.  
    ///
    /// lang_id: the language id of the code.  
    ///         For example, the language id of C++ is 73.  
    ///        You can get the language id from the submit page.  
    ///
    /// Return the submit id of the submit request.  
    fn submit(&mut self, identifier: &str, code: &str, lang_id: &str) -> Result<String, String> {
        let info: Vec<&str> = identifier.split("_").collect();
        if info.len() != 2 {
            return Err(String::from("Invalid identifier."));
        }
        let contest_id = info[0];
        let problem_id = info[1];
        let submit_page_url = format!("https://codeforces.com/contest/{}/submit", contest_id);
        let submit_page = match self.client.get(&submit_page_url) {
            Ok(page) => page,
            Err(err) => {
                return Err(String::from("unable to get submit page, ") + err.as_str());
            }
        };
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("csrf_token"), Self::get_csrf(&submit_page));
        params.insert(String::from("ftaa"), Self::get_ftaa());
        params.insert(String::from("bfaa"), Self::get_bfaa());
        params.insert(
            String::from("action"),
            String::from("submitSolutionFormSubmitted"),
        );
        params.insert(
            String::from("submittedProblemIndex"),
            problem_id.to_string(),
        );
        params.insert(String::from("submittedProblemCode"), String::from("4_A"));
        params.insert(String::from("programTypeId"), lang_id.to_string());
        params.insert(String::from("source"), code.to_string());
        params.insert(String::from("tabSize"), String::from("4"));
        params.insert(String::from("_tta"), String::from("176"));
        let submit_url = format!(
            "{}?csrf_token={}",
            submit_page_url,
            Self::get_csrf(&submit_page)
        );
        let resp = match self.client.post_form(&submit_url, &params) {
            Ok(resp) => resp,
            Err(err) => {
                return Err(String::from("Submit failed, ") + err.as_str());
            }
        };
        if resp.contains("You have submitted exactly the same code before") {
            return Err(String::from(
                "Submit failed, you have submitted exactly the same code before.",
            ));
        }
        return Ok(resp);
    }

    /// Check if the user is logged in.
    fn is_login(&mut self) -> Result<String, bool> {
        let main_page = self.client.get("https://codeforces.com").unwrap();
        let re = match Regex::new(r#"handle = "([\s\S]+?)""#) {
            Ok(re) => re,
            Err(_) => return Err(false),
        };
        let caps = match re.captures(main_page.as_str()) {
            Some(caps) => caps,
            None => return Err(false),
        };
        return Ok(caps[1].to_string());
    }

    /// Login to the platform.
    fn login(&mut self, username: &str, password: &str) -> String {
        let login_page = self.client.get("https://codeforces.com").unwrap();
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert(String::from("csrf_token"), Self::get_csrf(&login_page));
        params.insert(String::from("action"), String::from("enter"));
        params.insert(String::from("ftaa"), Self::get_ftaa());
        params.insert(String::from("bfaa"), Self::get_bfaa());
        params.insert(String::from("handleOrEmail"), String::from(username));
        params.insert(String::from("password"), String::from(password));
        params.insert(String::from("_tta"), String::from("176"));
        params.insert(String::from("remember"), String::from("on"));
        let resp = match self
            .client
            .post_form("https://codeforces.com/enter", &params)
        {
            Ok(resp) => resp,
            Err(err) => {
                println!("{}", err);
                return String::from("");
            }
        };
        return resp;
    }
    // TODO: get test cases
    fn get_test_cases(&mut self, identifier: &str) -> String {
        let _ = identifier;
        String::from("Codeforces get_test_cases")
    }
}

impl Codeforces {
    #[allow(unused)]
    fn new(cookies: &str) -> Self {
        let endpoint = String::from("https://codeforces.com");
        Self {
            client: HttpClient::new(cookies, &endpoint),
        }
    }
    fn get_bfaa() -> String {
        String::from("f1b3f18c715565b589b7823cda7448ce")
    }
    fn get_ftaa() -> String {
        random_str::get_string(18, true, false, true, false)
    }

    fn get_csrf(body: &str) -> String {
        let re = match Regex::new(r#"csrf='(.+?)'"#) {
            Ok(re) => re,
            Err(_) => return String::from(""),
        };
        match re.captures(body) {
            Some(caps) => caps[1].to_string(),
            None => String::from(""),
        }
    }
    fn to_hex_bytes(input: &str) -> [u8; 16] {
        let mut arr = [0; 32];
        for (i, c) in input.chars().enumerate() {
            arr[i] = c as u8;
        }
        let bytes = hex::decode(arr).unwrap();
        let mut output = [0u8; 16];
        output.copy_from_slice(&bytes);
        return output;
    }
    #[allow(unused)]
    fn get_rcpc(body: &str) -> String {
        if body.contains("Redirecting... Please, wait.") {
            return String::from("");
        }
        let re = match Regex::new(
            r#"var a=toNumbers\("([0-9a-f]*)"\),b=toNumbers\("([0-9a-f]*)"\),c=toNumbers\("([0-9a-f]*)"\);"#,
        ) {
            Ok(re) => re,
            Err(_) => return String::from(""),
        };
        let caps = match re.captures(body) {
            Some(caps) => caps,
            None => return String::from(""),
        };
        let key = Self::to_hex_bytes(caps[1].to_string().as_str());
        let iv = Self::to_hex_bytes(caps[2].to_string().as_str());
        let mut blk = Self::to_hex_bytes(caps[3].to_string().as_str()).into();
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
        Aes128CbcDec::new(&key.into(), &iv.into()).decrypt_block_mut(&mut blk);
        return hex::encode(blk);
    }

    /// Check if the contest is a regular contest.
    /// distinguish regular contest and gym contest.
    #[allow(unused)]
    fn is_regular_contest(identifier: &str) -> bool {
        return false;
    }
    #[allow(unused)]
    fn parse_recent_submit_id(resp: &str) -> Option<String>{
        let soup = Soup::new(resp);
        return Some(String::from(""));
    }
}

#[test]
fn test_parse_recent_submit_id(){
    dotenv::dotenv().ok();
}