use std::collections::HashMap;
mod builder;
mod parser;
mod utility;
mod constants;
use crate::database::CONFIG_DB;
use crate::misc::http_client::HttpClient;
use crate::model::Contest;
use crate::model::Platform;
use crate::model::SubmissionInfo;
use crate::model::TestCase;
use crate::traits::OnlineJudge;
use crate::model::PlatformLanguage;
use builder::UrlBuilder;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use regex::Regex;

use self::parser::HtmlParser;
use self::utility::Utility;
pub struct Codeforces {
    pub client: HttpClient,
    pub username: String,
    pub password: String,
}
impl Drop for Codeforces {
    fn drop(&mut self) {
        let _ = self.save_cookies();
    }
}
impl OnlineJudge for Codeforces {
    /// Submit code to the platform.  
    ///
    /// problem_identifier: the identifier of the problem.  
    ///             For example, the identifier of the problem <https://codeforces.com/problemset/problem/4/A> is 4_A.  
    ///
    /// code: the code to submit.  
    ///
    /// lang_id: the language id of the code.  
    ///         For example, the language id of C++ is 73.  
    ///        You can get the language id from the submit page.  
    ///
    /// Return the submit id of the submit request.  
    fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String> {
        let info: Vec<&str> = problem_identifier.split("_").collect();
        if info.len() != 2 {
            return Err(String::from("Invalid identifier."));
        }
        let contest_id = info[0];
        let problem_id = info[1];
        let submit_page_url = UrlBuilder::build_submit_page_url(contest_id);
        let submit_page = match self.client.get(&submit_page_url) {
            Ok(page) => page,
            Err(err) => {
                return Err(String::from("unable to get submit page, ") + err.as_str());
            }
        };
        let mut params: HashMap<&str, &str> = HashMap::new();
        let csrf_token = match Utility::get_csrf(&submit_page) {
            Ok(csrf_token) => csrf_token,
            Err(info) => {
                return Err(String::from("Submit failed, ") + info.as_str());
            }
        };
        let ftaa = Utility::get_ftaa();
        let bfaa = Utility::get_bfaa();
        params.insert("csrf_token", &csrf_token);
        params.insert("ftaa", &ftaa);
        params.insert("bfaa", &bfaa);
        params.insert("action", "submitSolutionFormSubmitted");
        params.insert("submittedProblemIndex", problem_id);
        params.insert("programTypeId", lang_id);
        params.insert("source", code);
        params.insert("tabSize", "4");
        params.insert("_tta", "176");
        let submit_url = UrlBuilder::build_submit_url(contest_id, &params["csrf_token"]);
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
        return HtmlParser::parse_recent_submission(&resp);
    }

    /// Check if the user is logged in.
    fn is_login(&mut self) -> Result<String, String> {
        let main_page = self.client.get("https://codeforces.com").unwrap();
        let re = match Regex::new(r#"handle = "([\s\S]+?)""#) {
            Ok(re) => re,
            Err(_) => return Err(String::from("Create regex failed.")),
        };
        let caps = match re.captures(main_page.as_str()) {
            Some(caps) => caps,
            None => return Err(String::from("Can't find handle.")),
        };
        return Ok(caps[1].to_string());
    }

    /// Login to the platform.
    fn login(&mut self, username: &str, password: &str) -> Result<String, String> {
        self.username = String::from(username);
        let login_page = match self.client.get(&UrlBuilder::build_index_url()) {
            Ok(login_page) => login_page,
            Err(info) => {
                return Err(info);
            }
        };
        let mut params: HashMap<&str, &str> = HashMap::new();
        let csrf_token = match Utility::get_csrf(&login_page) {
            Ok(csrf_token) => csrf_token,
            Err(info) => {
                return Err(String::from("Login failed, ") + info.as_str());
            }
        };
        let ftaa = Utility::get_ftaa();
        let bfaa = Utility::get_bfaa();

        params.insert("csrf_token", &csrf_token);
        params.insert("action", "enter");
        params.insert("ftaa", &ftaa);
        params.insert("bfaa", &bfaa);
        params.insert("handleOrEmail", username);
        params.insert("password", password);
        params.insert("_tta", "176");
        params.insert("remember", "on");
        return match self
            .client
            .post_form(&UrlBuilder::build_login_url(), &params)
        {
            Ok(resp) => Ok(resp),
            Err(err) => Err(err),
        };
    }
    /// Get test cases
    fn get_test_cases(&mut self, problem_url: &str) -> Result<Vec<TestCase>, String> {
        let resp = match self.client.get(problem_url) {
            Ok(resp) => resp,
            Err(err) => {
                return Err(String::from("Get problem page failed, ") + err.as_str());
            }
        };
        return HtmlParser::parse_test_cases(&resp);
    }

    fn retrive_result(
        &mut self,
        problem_identifier: &str,
        submission_id: &str,
    ) -> Result<SubmissionInfo, String> {
        let info: Vec<&str> = problem_identifier.split("_").collect();
        if info.len() != 2 {
            return Err(String::from("Invalid identifier."));
        }
        let contest_id = info[0];
        let problem_id = info[1];
        let url = UrlBuilder::build_submission_url(contest_id, submission_id);
        let resp = match self.client.get(&url) {
            Ok(resp) => resp,
            Err(info) => {
                return Err(info);
            }
        };
        return HtmlParser::parse_submission_page(submission_id, contest_id, problem_id, &resp);
    }

    fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<[String; 2]>, String> {
        let problem_list_url = UrlBuilder::build_problem_list_url(contest_identifier);
        let resp = match self.client.get(&problem_list_url) {
            Ok(resp) => resp,
            Err(info) => {
                return Err(info);
            }
        };
        return HtmlParser::parse_problem_list(contest_identifier, &resp);
    }

    fn get_contest(&mut self, contest_identifier: &str) -> Result<Contest, String> {
        let contest_url = UrlBuilder::build_contest_url(contest_identifier);
        let resp = match self.client.get(&contest_url) {
            Ok(resp) => resp,
            Err(info) => {
                return Err(info);
            }
        };
        return HtmlParser::parse_contest(contest_identifier, &resp);
    }

    fn save_cookies(&mut self) -> Result<(), String> {
        if !self.username.is_empty() {
            return CONFIG_DB.save_cookies(
                Platform::Codeforces,
                &self.username,
                &self.client.save_cookies(),
            );
        }
        return Ok(());
    }

    fn get_platform_languages() -> Vec<PlatformLanguage> {
        let mut vec = Vec::new();
        for (id, description, language) in constants::LANG.iter() {
            vec.push(PlatformLanguage {
                language: *language,
                platform: Platform::Codeforces,
                id: String::from(*id),
                description: String::from(*description),
            });
        }
        return vec;
    }
}

impl Codeforces {
    pub fn new() -> Result<Self, String> {
        let endpoint = String::from("https://codeforces.com");
        let account_info = match CONFIG_DB.get_default_account(Platform::Codeforces) {
            Ok(account_info) => account_info,
            Err(info) => return Err(info),
        };
        return Ok(Self::create(
            &account_info.username,
            &account_info.password,
            &account_info.cookies,
            &endpoint,
        ));
    }
    pub fn create(username: &str, password: &str, cookies: &str, endpoint: &str) -> Self {
        Self {
            client: HttpClient::new(cookies, &endpoint),
            username: String::from(username),
            password: String::from(password),
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
}

#[test]
#[ignore = "reason: need login"]
fn test_login() {
    dotenv::dotenv().ok();
    let mut cf = match Codeforces::new() {
        Ok(cf) => cf,
        Err(_) => {
            return;
        }
    };
    let username = dotenv::var("CODEFORCES_USERNAME").unwrap();
    let password = dotenv::var("CODEFORCES_PASSWORD").unwrap();
    let resp = cf.login(&username, &password);
    match resp {
        Ok(_) => {
            println!("Login success.");
        }
        Err(info) => {
            println!("Login failed, {}", info);
        }
    }
}
