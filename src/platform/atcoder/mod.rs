use crate::database::CONFIG_DB;
use crate::utility::http_client::HttpClient;
use crate::model::PlatformLanguage;
use crate::model::{Contest, SubmissionInfo};
use crate::model::{Platform, TestCase};
use crate::traits::OnlineJudge;
mod builder;
mod constants;
mod parser;
mod utility;
use builder::UrlBuilder;

use self::parser::HtmlParser;
use self::utility::Utility;
pub struct AtCoder {
    pub client: HttpClient,
    pub username: String,
    pub password: String,
}
impl Drop for AtCoder {
    fn drop(&mut self) {
        let _ = self.save_cookies();
    }
}
impl OnlineJudge for AtCoder {
    fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String> {
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        log::info!("vec: {:?}", vec);
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_id = vec[0];

        let submit_page_url = UrlBuilder::build_submit_page_url(contest_id);
        let resp = match self.client.get(&submit_page_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };

        let csrf_token = match Utility::get_csrf(&resp) {
            Some(token) => token,
            None => {
                return Err(String::from("Failed to get csrf token."));
            }
        };
        let mut data = std::collections::HashMap::new();
        data.insert("data.TaskScreenName", problem_identifier);
        data.insert("data.LanguageId", lang_id);
        data.insert("sourceCode", code);
        data.insert("csrf_token", &csrf_token);
        let submit_url = UrlBuilder::build_submit_url(contest_id);
        let resp = match self.client.post_form(&submit_url, &data) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return HtmlParser::parse_recent_submission_id(&resp);
    }

    fn is_login(&mut self) -> Result<String, String> {
        let resp = match self.client.get(&UrlBuilder::build_index_url()) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        if resp.contains("</span> Sign Out</a></li>") {
            return Ok(String::from("You have logged in."));
        } else {
            return Err(String::from("You have not logged in."));
        }
    }

    fn login(&mut self, username: &str, password: &str) -> Result<String, String> {
        self.username = String::from(username);
        let login_page_url = UrlBuilder::build_login_page_url();
        let resp = match self.client.get(&login_page_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        let csrf_token = match Utility::get_csrf(&resp) {
            Some(token) => token,
            None => {
                return Err(String::from("Failed to get csrf token."));
            }
        };
        let mut data = std::collections::HashMap::new();
        data.insert("username", username);
        data.insert("password", password);
        data.insert("csrf_token", &csrf_token);
        let login_url = UrlBuilder::build_login_url();
        let _ = match self.client.post_form(&login_url, &data) {
            Ok(_) => {}
            Err(info) => return Err(info),
        };
        return self.is_login();
    }

    /// Get test cases from AtCoder
    fn get_test_cases(&mut self, problem_url: &str) -> Result<Vec<TestCase>, String> {
        let resp = match self.client.get(&problem_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return HtmlParser::parse_test_cases(&resp);
    }
    fn retrive_result(
        &mut self,
        problem_identifier: &str,
        submission_id: &str,
    ) -> Result<SubmissionInfo, String> {
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_identifier = vec[0];
        let submission_url = UrlBuilder::build_submission_url(contest_identifier, submission_id);
        let resp = match self.client.get(&submission_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return HtmlParser::parse_submission_page(problem_identifier, submission_id, &resp);
    }

    fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<[String; 2]>, String> {
        let problem_list_url = UrlBuilder::build_problem_list_url(contest_identifier);
        let resp = match self.client.get(&problem_list_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return HtmlParser::parse_problem_list(contest_identifier, &resp);
    }

    fn get_contest(&mut self, contest_identifier: &str) -> Result<Contest, String> {
        let contest_url = UrlBuilder::build_contest_url(contest_identifier);
        let resp = match self.client.get(&contest_url) {
            Ok(resp) => resp,
            Err(info) => return Err(info),
        };
        return HtmlParser::parse_contest(contest_identifier, &resp);
    }

    fn save_cookies(&mut self) -> Result<(), String> {
        if !self.username.is_empty() {
            return CONFIG_DB.save_cookies(
                Platform::AtCoder,
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
                platform: Platform::AtCoder,
                id: String::from(*id),
                description: String::from(*description),
            });
        }
        return vec;
    }
}
impl AtCoder {
    pub fn new() -> Result<Self, String> {
        let endpoint = String::from("https://atcoder.jp");
        let account_info = match CONFIG_DB.get_default_account(Platform::AtCoder) {
            Ok(account_info) => account_info,
            Err(info) => {
                return Err(info);
            }
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
}
