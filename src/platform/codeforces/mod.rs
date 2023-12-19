use std::collections::HashMap;
mod constants;
mod parser;
mod utility;
use crate::model::Platform;
use crate::model::PlatformLanguage;
use crate::model::PostSubmissionInfo;
use crate::model::TestCase;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use regex::Regex;

use self::parser::HtmlParser;
use self::utility::Utility;

use super::traits::OnlineJudgeBehavior;
pub struct Codeforces;

impl OnlineJudgeBehavior for Codeforces {
    fn build_endpoint_url() -> String {
        return String::from("https://codeforces.com");
    }
    fn build_check_login_url() -> String {
        return String::from("https://codeforces.com");
    }

    fn check_login(resp: &str) -> bool {
        let re = match Regex::new(r#"handle = "([\s\S]+?)""#) {
            Ok(re) => re,
            Err(_) => return false,
        };
        match re.captures(resp) {
            Some(_) => true,
            None => false,
        }
    }

    fn build_login_page_url() -> String {
        return String::from("https://codeforces.com/enter");
    }

    fn build_login_url() -> String {
        return String::from("https://codeforces.com/enter");
    }

    fn build_login_request_data(
        username: &str,
        password: &str,
        csrf_token: &str,
    ) -> HashMap<String, String> {
        let mut data = HashMap::new();
        let ftaa = Utility::get_ftaa();
        let bfaa = Utility::get_bfaa();
        data.insert("csrf_token".to_string(), csrf_token.to_string());
        data.insert("action".to_string(), "enter".to_string());
        data.insert("ftaa".to_string(), ftaa);
        data.insert("bfaa".to_string(), bfaa);
        data.insert("handleOrEmail".to_string(), username.to_string());
        data.insert("password".to_string(), password.to_string());
        data.insert("_tta".to_string(), "176".to_string());
        data.insert("remember".to_string(), "on".to_string());
        return data;
    }

    fn build_contest_url(contest_identifier: &str) -> String {
        return format!("https://codeforces.com/contests/{}", contest_identifier);
    }

    fn build_problem_list_url(contest_identifier: &str) -> String {
        return format!("https://codeforces.com/contest/{}", contest_identifier);
    }

    fn build_submit_page_url(contest_identifier: &str) -> String {
        return format!(
            "https://codeforces.com/contest/{}/submit",
            contest_identifier
        );
    }

    fn build_submit_url(contest_identifier: &str, _csrf_token: &str) -> String {
        return format!(
            "https://codeforces.com/contest/{}/submit",
            contest_identifier
        );
    }

    fn build_submission_url(contest_identifier: &str, submission_id: &str) -> String {
        return format!(
            "https://codeforces.com/contest/{}/submission/{}",
            contest_identifier, submission_id
        );
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
        return HtmlParser::parse_submission_page(
            submission_id,
            contest_identifier,
            problem_identifier,
            resp,
        );
    }

    fn parse_recent_submission_id(resp: &str) -> Result<String, String> {
        return HtmlParser::parse_recent_submission_id(resp);
    }


    fn parse_contest(contest_identifier: &str, resp: &str) -> Result<crate::model::Contest, String> {
        return HtmlParser::parse_contest(contest_identifier, resp);
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
        let ftaa = Utility::get_ftaa();
        let bfaa = Utility::get_bfaa();
        data.insert("csrf_token".to_string(), csrf_token.to_string());
        data.insert("action".to_string(), "submitSolutionFormSubmitted".to_string());
        data.insert("ftaa".to_string(), ftaa);
        data.insert("bfaa".to_string(), bfaa);
        data.insert("submittedProblemIndex".to_string(), problem_identifier.to_string());
        data.insert("programTypeId".to_string(), lang_id.to_string());
        data.insert("source".to_string(), code.to_string());
        data.insert("_tta".to_string(), "176".to_string());
        return data;
    }
}

impl Codeforces {
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
