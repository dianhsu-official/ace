use std::collections::HashMap;

use crate::model::PlatformLanguage;
use crate::model::PostSubmissionInfo;
use crate::model::TestCase;
use crate::model::{Platform, Contest};
mod constants;
mod parser;
mod utility;

use self::parser::HtmlParser;
use self::utility::Utility;

use super::traits::OnlineJudgeBehavior;
pub struct AtCoder;

impl OnlineJudgeBehavior for AtCoder {
    fn build_endpoint_url() -> String {
        return String::from("https://atcoder.jp");
    }
    fn build_check_login_url() -> String {
        return String::from("https://atcoder.jp");
    }

    fn check_login(resp: &str) -> bool {
        return resp.contains(r#"</span> Sign Out</a></li>"#);
    }

    fn build_login_page_url() -> String {
        return String::from("https://atcoder.jp/login");
    }

    fn build_login_url() -> String {
        return String::from("https://atcoder.jp/login");
    }

    fn build_login_request_data(
        username: &str,
        password: &str,
        csrf_token: &str,
    ) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("username".to_string(), username.to_string());
        data.insert("password".to_string(), password.to_string());
        data.insert("csrf_token".to_string(), csrf_token.to_string());
        return data;
    }

    fn build_contest_url(contest_identifier: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}",
            contest_identifier
        ));
    }

    fn build_problem_list_url(contest_identifier: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/tasks",
            contest_identifier
        ));
    }

    fn build_submit_page_url(contest_identifier: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/submit",
            contest_identifier
        ));
    }

    fn build_submit_url(contest_identifier: &str, _csrf_token: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/submit",
            contest_identifier
        ));
    }

    fn build_submission_url(contest_identifier: &str, submission_id: &str) -> String {
        return String::from(format!(
            "https://atcoder.jp/contests/{}/submissions/{}",
            contest_identifier, submission_id
        ));
    }

    fn parse_problem_list(
        contest_identifier: &str,
        resp: &str,
    ) -> Result<Vec<[String; 2]>, String> {
        return HtmlParser::parse_problem_list(contest_identifier, resp);
    }

    fn parse_test_cases(resp: &str) -> Result<Vec<TestCase>, String> {
        return HtmlParser::parse_test_cases(resp);
    }

    fn parse_submission_page(
        contest_identifier: &str,
        problem_identifier: &str,
        submission_id: &str,
        resp: &str,
    ) -> Result<PostSubmissionInfo, String> {
        return HtmlParser::parse_submission_page(contest_identifier, problem_identifier, submission_id, resp);
    }

    fn parse_recent_submission_id(resp: &str) -> Result<String, String> {
        return HtmlParser::parse_recent_submission_id(resp);
    }

    fn parse_contest(
        contest_identifier: &str,
        resp: &str,
    ) -> Result<Contest, String> {
        return HtmlParser::parse_contest(contest_identifier, resp);
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

    fn get_csrf_token(resp: &str) -> Result<String, String> {
        return Utility::get_csrf(resp);
    }

    fn build_submit_request_data(
        _contest_identifier: &str,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
        csrf_token: &str,
    ) -> HashMap<String, String> {
        let mut data = HashMap::new();
        data.insert("data.TaskScreenName".to_string(), problem_identifier.to_string());
        data.insert("data.LanguageId".to_string(), lang_id.to_string());
        data.insert("sourceCode".to_string(), code.to_string());
        data.insert("csrf_token".to_string(), csrf_token.to_string());
        return data;
    }
}

