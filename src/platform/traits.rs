use crate::model::{PostSubmissionInfo, TestCase, Contest, PlatformLanguage};
use std::collections::HashMap;

pub trait OnlineJudgeBehavior {
    fn build_check_login_url() -> String;
    fn check_login(resp: &str) -> bool;

    fn build_endpoint_url() -> String;
    fn build_login_page_url() -> String;
    fn build_login_url() -> String;
    fn build_login_request_data(
        username: &str,
        password: &str,
        csrf_token: &str,
    ) -> HashMap<String, String>;

    fn build_submit_request_data(
        contest_identifier: &str,
        problem_identifier: &str,
        code: &str,
        lang_id: &str,
        csrf_token: &str,
    ) -> HashMap<String, String>;

    fn build_contest_url(contest_identifier: &str) -> String;
    fn build_problem_list_url(contest_identifier: &str) -> String;
    fn build_submit_page_url(contest_identifier: &str) -> String;
    fn build_submit_url(contest_identifier: &str, csrf_token: &str) -> String;
    fn build_submission_url(contest_identifier: &str, submission_id: &str) -> String;

    fn parse_problem_list(contest_identifier: &str, resp: &str)
        -> Result<Vec<[String; 2]>, String>;
    fn parse_test_cases(resp: &str) -> Result<Vec<TestCase>, String>;
    fn parse_submission_page(
        contest_identifier: &str,
        problem_identifier: &str,
        submission_id: &str,
        resp: &str,
    ) -> Result<PostSubmissionInfo, String>;
    fn parse_recent_submission_id(resp: &str) -> Result<String, String>;
    fn parse_contest(contest_identifier: &str, resp: &str) -> Result<Contest, String>;

    fn get_platform_languages() -> Vec<PlatformLanguage>;
    fn get_csrf_token(resp: &str) -> Result<String, String>;
}
