use crate::library::OnlineJudge;
use crate::misc::http_client::HttpClient;
use crate::model::{Contest, SubmissionInfo};
mod builder;
mod config;
mod parser;
mod utility;
use builder::UrlBuilder;

use self::parser::HtmlParser;
use self::utility::Utility;
pub struct AtCoder {
    pub client: HttpClient,
    pub host: String,
}
impl OnlineJudge for AtCoder {
    fn submit(
        &mut self,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
    ) -> Result<String, String> {
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
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
    /// Success: Vec<[String; 2]> where [0] is input and [1] is output
    fn get_test_cases(&mut self, problem_identifier: &str) -> Result<Vec<[String; 2]>, String> {
        let vec = problem_identifier.split("_").collect::<Vec<_>>();
        if vec.len() != 2 {
            return Err(String::from("Invalid problem identifier."));
        }
        let contest_identifier = vec[0];
        let problem_url = UrlBuilder::build_problem_url(contest_identifier, problem_identifier);
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

    fn get_problems(&mut self, contest_identifier: &str) -> Result<Vec<String>, String> {
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
}
impl AtCoder {
    #[allow(unused)]
    pub fn new(cookies: &str) -> Self {
        return Self {
            client: HttpClient::new(cookies, "https://atcoder.jp"),
            host: String::from("https://atcoder.jp"),
        };
    }
}
